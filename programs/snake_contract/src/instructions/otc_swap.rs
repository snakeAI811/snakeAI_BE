use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Approve};
use crate::state::{UserClaim, UserRole, PatronStatus, OtcSwap};
use crate::errors::SnakeError;
use crate::events::{SwapInitiated, SwapCompleted, SwapCancelled};

// OtcSwap struct is defined in state/otc_swap.rs

#[derive(Accounts)]
#[instruction(token_amount: u64, buyer_role_required: UserRole)] 
pub struct InitiateOtcSwap<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    // Verify seller has tokens to sell and can sell them
    #[account(
        mut,
        constraint = seller_claim.initialized @ SnakeError::Unauthorized,
        constraint = seller_claim.role != UserRole::None @ SnakeError::Unauthorized,
        seeds = [b"user_claim", seller.key().as_ref()],
        bump,
    )]
    pub seller_claim: Account<'info, UserClaim>,
    
    #[account(
        init,
        payer = seller,
        space = 8 + 32 + 33 + 8 + 8 + 8 + 1 + 8 + 1 + 1 + 1, // Updated space for new fields
        seeds = [b"otc_swap", seller.key().as_ref()],
        bump,
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key(),
        constraint = seller_token_account.amount >= token_amount @ SnakeError::InsufficientFunds,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn initiate_otc_swap(
    ctx: Context<InitiateOtcSwap>, 
    token_amount: u64, 
    sol_rate: u64,
    buyer_rebate: u64,
    buyer_role_required: UserRole
) -> Result<()> {
    let seller_claim = &mut ctx.accounts.seller_claim;
    let current_time = Clock::get()?.unix_timestamp;
    
    require!(token_amount > 0, CustomError::InvalidAmount);
    require!(sol_rate > 0, CustomError::InvalidRate);
    require!(buyer_rebate <= 1000, CustomError::InvalidRebate); // Max 10% rebate
    
    // Validate seller can sell tokens
    match seller_claim.role {
        UserRole::Patron => {
            // Patrons have special restrictions
            require!(
                seller_claim.patron_status == PatronStatus::Approved,
                SnakeError::PatronNotApproved
            );
            
            // Check if patron is in lock period (can't sell during commitment)
            require!(!seller_claim.is_locked(), SnakeError::TokensLocked);
            
            // Check 6-month commitment period
            let six_months_seconds = 6 * 30 * 24 * 60 * 60;
            let commitment_end = seller_claim.patron_approval_timestamp + six_months_seconds;
            
            // If still in commitment period, only allow transfer to other patrons
            if current_time < commitment_end {
                require!(
                    buyer_role_required == UserRole::Patron,
                    SnakeError::PatronTransferRestricted
                );
            }
            
            // Mark early sale if within commitment period
            if current_time < commitment_end {
                seller_claim.sold_early = true;
            }
        },
        UserRole::Staker => {
            // Stakers can't sell locked tokens
            require!(!seller_claim.is_locked(), SnakeError::TokensLocked);
        },
        UserRole::None => {
            // Regular users can sell freely
        }
    }
    
    // Validate buyer role requirement
    require!(
        buyer_role_required == UserRole::Patron || 
        buyer_role_required == UserRole::Staker ||
        buyer_role_required == UserRole::None, // Allow anyone
        CustomError::InvalidRole
    );
    
    let swap = &mut ctx.accounts.otc_swap;
    swap.seller = ctx.accounts.seller.key();
    swap.buyer = None;
    swap.token_amount = token_amount;
    swap.sol_rate = sol_rate;
    swap.buyer_rebate = buyer_rebate;
    swap.is_active = true;
    swap.created_at = current_time;
    swap.seller_role = seller_claim.role.clone();
    let buyer_role_for_emit = buyer_role_required.clone();
    swap.buyer_role_required = buyer_role_required;
    swap.bump = ctx.bumps.otc_swap;
    
    // Approve the OTC swap PDA to transfer tokens
    let cpi_accounts = Approve {
        to: ctx.accounts.seller_token_account.to_account_info(),
        delegate: ctx.accounts.otc_swap.to_account_info(),
        authority: ctx.accounts.seller.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::approve(cpi_ctx, token_amount)?;
    
    emit!(SwapInitiated {
        seller: ctx.accounts.seller.key(),
        otc_swap: ctx.accounts.otc_swap.key(),
        token_amount,
        sol_rate,
        buyer_rebate,
        buyer_role_required: buyer_role_for_emit,
    });
    
    Ok(())
}

#[derive(Accounts)]
pub struct AcceptOtcSwap<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    // Verify buyer has required role and can buy
    #[account(
        mut,
        constraint = buyer_claim.initialized @ SnakeError::Unauthorized,
        seeds = [b"user_claim", buyer.key().as_ref()],
        bump,
    )]
    pub buyer_claim: Account<'info, UserClaim>,
    
    /// CHECK: Validated through PDA seeds and constraints
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"otc_swap", seller.key().as_ref()],
        bump = otc_swap.bump,
        constraint = otc_swap.seller == seller.key() @ SnakeError::Unauthorized,
        constraint = otc_swap.is_active @ CustomError::SwapInactive,
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key(),
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = buyer_token_account.owner == buyer.key(),
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    // Treasury collects fees/rebates
    #[account(
        mut,
        seeds = [b"treasury"],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn accept_otc_swap(ctx: Context<AcceptOtcSwap>) -> Result<()> {
    let buyer_claim = &mut ctx.accounts.buyer_claim;
    
    // Extract all values we need BEFORE taking mutable borrow
    let token_amount = ctx.accounts.otc_swap.token_amount;
    let sol_rate = ctx.accounts.otc_swap.sol_rate;
    let buyer_rebate = ctx.accounts.otc_swap.buyer_rebate;
    let is_active = ctx.accounts.otc_swap.is_active;
    let buyer_option = ctx.accounts.otc_swap.buyer;
    let buyer_role_required = ctx.accounts.otc_swap.buyer_role_required.clone();
    let seller_role = ctx.accounts.otc_swap.seller_role.clone();
    let otc_swap_key = ctx.accounts.otc_swap.key();
    let seller_key = ctx.accounts.seller.key();
    let buyer_key = ctx.accounts.buyer.key();
    let bump = ctx.accounts.otc_swap.bump;
    
    require!(is_active, CustomError::SwapInactive);
    require!(buyer_option.is_none(), CustomError::SwapAlreadyAccepted);
    
    // Validate buyer meets role requirements
    match buyer_role_required {
        UserRole::Patron => {
            require!(
                buyer_claim.role == UserRole::Patron && 
                buyer_claim.patron_status == PatronStatus::Approved,
                SnakeError::OnlyApprovedPatrons
            );
        },
        UserRole::Staker => {
            require!(
                buyer_claim.role == UserRole::Staker || 
                (buyer_claim.role == UserRole::Patron && buyer_claim.patron_status == PatronStatus::Approved),
                SnakeError::Unauthorized
            );
        },
        UserRole::None => {
            // Anyone can buy
        },
    }
    
    // Additional validation for patron sellers - ensure buyer is also patron if required
    if seller_role == UserRole::Patron && buyer_role_required == UserRole::Patron {
        require!(
            buyer_claim.role == UserRole::Patron && 
            buyer_claim.patron_status == PatronStatus::Approved,
            SnakeError::PatronTransferRestricted
        );
    }
    
    // Calculate total payment and rebate
    let total_sol_payment = token_amount
        .checked_mul(sol_rate)
        .ok_or(CustomError::MathOverflow)?;
    
    let rebate_amount = total_sol_payment
        .checked_mul(buyer_rebate)
        .ok_or(CustomError::MathOverflow)?
        .checked_div(10000) // basis points
        .ok_or(CustomError::MathOverflow)?;
    
    let net_payment = total_sol_payment
        .checked_sub(rebate_amount)
        .ok_or(CustomError::MathOverflow)?;
    
    // Verify buyer has sufficient SOL
    require!(
        ctx.accounts.buyer.to_account_info().lamports() >= total_sol_payment,
        CustomError::InsufficientFunds
    );
    
    // NOW take mutable borrow and update swap state
    let swap = &mut ctx.accounts.otc_swap;
    swap.buyer = Some(buyer_key);
    swap.is_active = false;
    
    // Create PDA signer seeds
    let signer_seeds = &[
        b"otc_swap",
        seller_key.as_ref(),
        &[bump],
    ];
    let signer = &[&signer_seeds[..]];
    
    // Transfer tokens from seller to buyer
    let cpi_accounts = Transfer {
        from: ctx.accounts.seller_token_account.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.otc_swap.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), 
        cpi_accounts,
        signer,
    );
    
    token::transfer(cpi_ctx, token_amount)?;
    
    // Transfer SOL: buyer pays seller (minus rebate)
    **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? -= total_sol_payment;
    **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += net_payment;
    
    // Transfer rebate to treasury (or back to buyer as reward)
    if rebate_amount > 0 {
        **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? += rebate_amount;
    }
    
    emit!(SwapCompleted {
        seller: seller_key,
        buyer: buyer_key,
        otc_swap: otc_swap_key,
        token_amount,
        sol_payment: net_payment,
        rebate_amount,
    });
    
    Ok(())
}

#[derive(Accounts)]
pub struct CancelOtcSwap<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"otc_swap", seller.key().as_ref()],
        bump = otc_swap.bump,
        constraint = otc_swap.seller == seller.key() @ SnakeError::Unauthorized,
        constraint = otc_swap.is_active @ CustomError::SwapInactive,
        close = seller, // Return rent to seller
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn cancel_otc_swap(ctx: Context<CancelOtcSwap>) -> Result<()> {
    // Revoke token approval
    let cpi_accounts = anchor_spl::token::Revoke {
        source: ctx.accounts.seller_token_account.to_account_info(),
        authority: ctx.accounts.seller.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    anchor_spl::token::revoke(cpi_ctx)?;
    
    emit!(SwapCancelled {
        seller: ctx.accounts.seller.key(),
        otc_swap: ctx.accounts.otc_swap.key(),
    });
    
    Ok(())
}

// Events are defined in events.rs

#[error_code]
pub enum CustomError {
    #[msg("Swap is not active")] 
    SwapInactive,
    #[msg("Swap already accepted")]
    SwapAlreadyAccepted,
    #[msg("Invalid token amount")]
    InvalidAmount,
    #[msg("Invalid SOL rate")]
    InvalidRate,
    #[msg("Invalid rebate percentage")]
    InvalidRebate,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Invalid role for this operation")]
    InvalidRole,
}