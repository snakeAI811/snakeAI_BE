use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Approve};
use crate::state::{UserClaim, UserRole, PatronStatus};
use crate::errors::SnakeError;
use crate::events::{SwapInitiated, SwapCompleted, SwapCancelled};

// ========== MILESTONE 1: BASIC OTC SWAP SYSTEM ==========
// Simple Normal User → Patron token trading

#[account]
#[derive(InitSpace)]
pub struct BasicOtcSwap {
    pub seller: Pubkey,
    pub buyer: Option<Pubkey>,
    pub token_amount: u64,
    pub sol_rate: u64,           // SOL per token (in lamports)
    pub buyer_rebate: u64,       // Rebate percentage (basis points)
    pub is_active: bool,
    pub created_at: i64,
    pub seller_role: UserRole,
    pub buyer_role_required: UserRole,
    pub bump: u8,
}

// ========== INITIATE BASIC OTC SWAP ==========

#[derive(Accounts)]
#[instruction(token_amount: u64, sol_rate: u64, buyer_rebate: u64, buyer_role_required: UserRole)]
pub struct InitiateBasicOtcSwap<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        constraint = seller_claim.initialized @ SnakeError::Unauthorized,
        constraint = seller_claim.role == UserRole::None @ SnakeError::OnlyNormalUsersCanSell,
        seeds = [b"user_claim", seller.key().as_ref()],
        bump,
    )]
    pub seller_claim: Account<'info, UserClaim>,
    
    #[account(
        init,
        payer = seller,
        space = 8 + BasicOtcSwap::INIT_SPACE,
        seeds = [b"basic_otc_swap", seller.key().as_ref()],
        bump,
    )]
    pub otc_swap: Account<'info, BasicOtcSwap>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key(),
        constraint = seller_token_account.amount >= token_amount @ SnakeError::InsufficientFunds,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn initiate_basic_otc_swap(
    ctx: Context<InitiateBasicOtcSwap>,
    token_amount: u64,
    sol_rate: u64,
    buyer_rebate: u64,
    buyer_role_required: UserRole,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    require!(token_amount > 0, SnakeError::InvalidAmount);
    require!(sol_rate > 0, SnakeError::InvalidRate);
    require!(buyer_rebate <= 1000, SnakeError::InvalidRebate); // Max 10%
    require!(buyer_role_required == UserRole::Patron, SnakeError::OnlyPatronsCanBuy);
    
    // Ensure seller has unlocked tokens
    require!(!ctx.accounts.seller_claim.is_locked(), SnakeError::TokensLocked);
    
    let swap = &mut ctx.accounts.otc_swap;
    swap.seller = ctx.accounts.seller.key();
    swap.buyer = None;
    swap.token_amount = token_amount;
    swap.sol_rate = sol_rate;
    swap.buyer_rebate = buyer_rebate;
    swap.is_active = true;
    swap.created_at = current_time;
    swap.seller_role = ctx.accounts.seller_claim.role.clone();
    swap.buyer_role_required = buyer_role_required.clone();
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
        buyer_role_required: buyer_role_required,
        otc_swap: ctx.accounts.otc_swap.key(),
        token_amount,
        sol_rate,
        buyer_rebate,
    });
    
    Ok(())
}

// ========== ACCEPT BASIC OTC SWAP ==========

#[derive(Accounts)]
pub struct AcceptBasicOtcSwap<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(
        constraint = buyer_claim.initialized @ SnakeError::Unauthorized,
        constraint = buyer_claim.role == UserRole::Patron @ SnakeError::OnlyPatronsCanBuy,
        constraint = buyer_claim.patron_status == PatronStatus::Approved @ SnakeError::OnlyApprovedPatrons,
        seeds = [b"user_claim", buyer.key().as_ref()],
        bump,
    )]
    pub buyer_claim: Account<'info, UserClaim>,
    
    /// CHECK: Validated through PDA seeds and constraints
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"basic_otc_swap", seller.key().as_ref()],
        bump = otc_swap.bump,
        constraint = otc_swap.seller == seller.key() @ SnakeError::Unauthorized,
        constraint = otc_swap.is_active @ SnakeError::SwapInactive,
        constraint = otc_swap.buyer.is_none() @ SnakeError::SwapAlreadyAccepted,
        constraint = otc_swap.buyer_role_required == UserRole::Patron @ SnakeError::OnlyPatronsCanBuy,
    )]
    pub otc_swap: Account<'info, BasicOtcSwap>,
    
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
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn accept_basic_otc_swap(ctx: Context<AcceptBasicOtcSwap>) -> Result<()> {
    let token_amount = ctx.accounts.otc_swap.token_amount;
    let sol_rate = ctx.accounts.otc_swap.sol_rate;
    let buyer_rebate = ctx.accounts.otc_swap.buyer_rebate;
    let seller_key = ctx.accounts.seller.key();
    let buyer_key = ctx.accounts.buyer.key();
    let otc_swap_key = ctx.accounts.otc_swap.key();
    let bump = ctx.accounts.otc_swap.bump;
    
    // Prevent self-trading
    require!(seller_key != buyer_key, SnakeError::CannotBuyOwnSwap);
    
    // Calculate SOL payment with rebate
    let base_sol_payment = token_amount
        .checked_mul(sol_rate)
        .ok_or(SnakeError::MathOverflow)?;
    
    let rebate_amount = base_sol_payment
        .checked_mul(buyer_rebate)
        .ok_or(SnakeError::MathOverflow)?
        .checked_div(10000)
        .ok_or(SnakeError::MathOverflow)?;
    
    let final_sol_payment = base_sol_payment
        .checked_sub(rebate_amount)
        .ok_or(SnakeError::MathOverflow)?;
    
    // Verify buyer has sufficient SOL
    require!(
        ctx.accounts.buyer.to_account_info().lamports() >= final_sol_payment,
        SnakeError::InsufficientFunds
    );
    
    // Update swap state
    let swap = &mut ctx.accounts.otc_swap;
    swap.buyer = Some(buyer_key);
    swap.is_active = false;
    
    // Create PDA signer seeds
    let signer_seeds = &[
        b"basic_otc_swap",
        seller_key.as_ref(),
        &[bump],
    ];
    let signer = &[&signer_seeds[..]];
    
    // Transfer tokens from seller to buyer
    let transfer_accounts = Transfer {
        from: ctx.accounts.seller_token_account.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.otc_swap.to_account_info(),
    };
    
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer,
    );
    
    token::transfer(transfer_ctx, token_amount)?;
    
    // Transfer SOL from buyer to seller
    **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? -= final_sol_payment;
    **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += final_sol_payment;
    
    emit!(SwapCompleted {
        seller: seller_key,
        buyer: buyer_key,
        otc_swap: otc_swap_key,
        token_amount,
        sol_payment: final_sol_payment,
        rebate_amount,
    });
    
    Ok(())
}

// ========== CANCEL BASIC OTC SWAP ==========

#[derive(Accounts)]
pub struct CancelBasicOtcSwap<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"basic_otc_swap", seller.key().as_ref()],
        bump = otc_swap.bump,
        constraint = otc_swap.seller == seller.key() @ SnakeError::Unauthorized,
        constraint = otc_swap.is_active @ SnakeError::SwapInactive,
        constraint = otc_swap.buyer.is_none() @ SnakeError::SwapAlreadyAccepted,
    )]
    pub otc_swap: Account<'info, BasicOtcSwap>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key(),
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn cancel_basic_otc_swap(ctx: Context<CancelBasicOtcSwap>) -> Result<()> {
    let seller_key = ctx.accounts.seller.key();
    let otc_swap_key = ctx.accounts.otc_swap.key();
    let token_amount = ctx.accounts.otc_swap.token_amount;
    let bump = ctx.accounts.otc_swap.bump;
    
    // Update swap state
    let swap = &mut ctx.accounts.otc_swap;
    swap.is_active = false;
    
    // Create PDA signer seeds for revoking approval
    let signer_seeds = &[
        b"basic_otc_swap",
        seller_key.as_ref(),
        &[bump],
    ];
    let signer = &[&signer_seeds[..]];
    
    // Revoke token approval (set to 0)
    let approve_accounts = Approve {
        to: ctx.accounts.seller_token_account.to_account_info(),
        delegate: ctx.accounts.otc_swap.to_account_info(),
        authority: ctx.accounts.seller.to_account_info(),
    };
    
    let approve_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), approve_accounts);
    token::approve(approve_ctx, 0)?;
    
    emit!(SwapCancelled {
        seller: seller_key,
        otc_swap: otc_swap_key,
    });
    
    Ok(())
}