use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Approve, Burn, Mint};
use crate::state::{UserClaim, UserRole, PatronStatus, OtcSwap, RewardPool};
use crate::errors::SnakeError;
use crate::events::{SwapInitiated, SwapCompleted, TokensBurned};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum SwapType {
    NormalToPatron,     // Normal user → Patron
    NormalToStaker,     // Normal user → Staker
    PatronToPatron,     // Patron → Patron (with restrictions)
    TreasuryBuyback,    // Treasury buyback
    AnyToAny,           // Open market
}

// ========== INITIATE OTC SWAP (ENHANCED) ==========

#[derive(Accounts)]
#[instruction(token_amount: u64, sol_rate: u64, buyer_rebate: u64, swap_type: SwapType)]
pub struct InitiateOtcSwapEnhanced<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        constraint = seller_claim.initialized @ SnakeError::Unauthorized,
        seeds = [b"user_claim", seller.key().as_ref()],
        bump,
    )]
    pub seller_claim: Account<'info, UserClaim>,
    
    #[account(
        init,
        payer = seller,
        space = 8 + OtcSwap::INIT_SPACE,
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

pub fn initiate_otc_swap_enhanced(
    ctx: Context<InitiateOtcSwapEnhanced>, 
    token_amount: u64, 
    sol_rate: u64,
    buyer_rebate: u64,
    swap_type: SwapType
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let seller_claim = &ctx.accounts.seller_claim;
    
    require!(token_amount > 0, SnakeError::InvalidAmount);
    require!(sol_rate > 0, SnakeError::InvalidRate);
    
    // Validate seller can create this type of swap
    match swap_type {
        SwapType::NormalToPatron => {
            require!(seller_claim.role == UserRole::None, SnakeError::OnlyNormalUsersCanSell);
        },
        SwapType::NormalToStaker => {
            require!(seller_claim.role == UserRole::None, SnakeError::OnlyNormalUsersCanSell);
        },
        SwapType::PatronToPatron => {
            require!(
                seller_claim.role == UserRole::Patron && 
                seller_claim.patron_status == PatronStatus::Approved,
                SnakeError::OnlyApprovedPatrons
            );
            
            // Check if patron is within 6-month commitment
            let six_months_in_seconds = 6 * 30 * 24 * 60 * 60;
            let commitment_end = seller_claim.patron_approval_timestamp + six_months_in_seconds;
            
            if current_time < commitment_end {
                // Mark as early sale
                // This will be handled in the accept function
            }
        },
        SwapType::TreasuryBuyback => {
            // Anyone can sell to treasury
        },
        SwapType::AnyToAny => {
            // Anyone can sell to anyone
        }
    }
    
    // Ensure seller has unlocked tokens
    require!(!seller_claim.is_locked(), SnakeError::TokensLocked);
    
    let swap = &mut ctx.accounts.otc_swap;
    
    // Initialize swap with basic data
    swap.seller = ctx.accounts.seller.key();
    swap.buyer = None;
    swap.token_amount = token_amount;
    swap.sol_rate = sol_rate;
    swap.buyer_rebate = buyer_rebate;
    swap.is_active = true;
    swap.created_at = current_time;
    swap.seller_role = seller_claim.role.clone();
    swap.bump = ctx.bumps.otc_swap;
    
    // Set buyer role requirement based on swap type
    match swap_type {
        SwapType::NormalToPatron => {
            swap.buyer_role_required = UserRole::Patron;
            swap.swap_type = crate::state::SwapType::NormalToPatron;
        },
        SwapType::NormalToStaker => {
            swap.buyer_role_required = UserRole::Staker;
            swap.swap_type = crate::state::SwapType::NormalToStaker;
        },
        SwapType::PatronToPatron => {
            swap.buyer_role_required = UserRole::Patron;
            swap.swap_type = crate::state::SwapType::PatronToPatron;
        },
        SwapType::TreasuryBuyback => {
            swap.buyer_role_required = UserRole::None;
            swap.swap_type = crate::state::SwapType::TreasuryBuyback;
        },
        SwapType::AnyToAny => {
            swap.buyer_role_required = UserRole::None;
            swap.swap_type = crate::state::SwapType::AnyToAny;
        }
    }
    
    // Store values before borrow
    let buyer_role_required = swap.buyer_role_required.clone();
    let buyer_rebate = swap.buyer_rebate;
    let otc_swap_key = ctx.accounts.otc_swap.key();
    
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
        buyer_role_required,
        otc_swap: otc_swap_key,
        token_amount,
        sol_rate,
        buyer_rebate,
    });
    
    Ok(())
}

// ========== ACCEPT OTC SWAP (PATRON-TO-PATRON) ==========

#[derive(Accounts)]
pub struct AcceptOtcSwapPatronToPatron<'info> {
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
        constraint = seller_claim.initialized @ SnakeError::Unauthorized,
        seeds = [b"user_claim", seller.key().as_ref()],
        bump,
    )]
    pub seller_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"otc_swap", seller.key().as_ref()],
        bump = otc_swap.bump,
        constraint = otc_swap.seller == seller.key() @ SnakeError::Unauthorized,
        constraint = otc_swap.is_active @ SnakeError::SwapInactive,
        constraint = otc_swap.buyer.is_none() @ SnakeError::SwapAlreadyAccepted,
        constraint = otc_swap.swap_type == crate::state::SwapType::PatronToPatron @ SnakeError::InvalidSwapType,
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
    
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn accept_otc_swap_patron_to_patron(ctx: Context<AcceptOtcSwapPatronToPatron>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let _token_amount = ctx.accounts.otc_swap.token_amount;
    let sol_rate = ctx.accounts.otc_swap.sol_rate;
    let seller_key = ctx.accounts.seller.key();
    let buyer_key = ctx.accounts.buyer.key();
    let otc_swap_key = ctx.accounts.otc_swap.key();
    let bump = ctx.accounts.otc_swap.bump;
    
    // Check if swap is expired
    require!(!ctx.accounts.otc_swap.is_expired(current_time), SnakeError::SwapExpired);
    
    // Calculate burn amount (20% for patron exits)
    let burn_amount = ctx.accounts.otc_swap.calculate_burn_amount();
    let net_tokens_to_buyer = ctx.accounts.otc_swap.calculate_net_tokens_after_burn();
    
    // Calculate SOL payment
    let total_sol_payment = net_tokens_to_buyer
        .checked_mul(sol_rate)
        .ok_or(SnakeError::MathOverflow)?;
    
    // Verify buyer has sufficient SOL
    require!(
        ctx.accounts.buyer.to_account_info().lamports() >= total_sol_payment,
        SnakeError::InsufficientFunds
    );
    
    // Update swap state
    let swap = &mut ctx.accounts.otc_swap;
    swap.buyer = Some(buyer_key);
    swap.is_active = false;
    
    // Mark seller as having sold early if within commitment period
    let seller_claim = &mut ctx.accounts.seller_claim;
    let six_months_in_seconds = 6 * 30 * 24 * 60 * 60;
    let commitment_end = seller_claim.patron_approval_timestamp + six_months_in_seconds;
    
    if current_time < commitment_end {
        seller_claim.sold_early = true;
    }
    
    // Create PDA signer seeds
    let signer_seeds = &[
        b"otc_swap",
        seller_key.as_ref(),
        &[bump],
    ];
    let signer = &[&signer_seeds[..]];
    
    // Transfer full token amount from seller to swap PDA first
    let _transfer_to_pda = Transfer {
        from: ctx.accounts.seller_token_account.to_account_info(),
        to: ctx.accounts.seller_token_account.to_account_info(), // Temporary, we'll burn from here
        authority: ctx.accounts.otc_swap.to_account_info(),
    };
    
    // Burn tokens (20% penalty)
    if burn_amount > 0 {
        let burn_accounts = Burn {
            mint: ctx.accounts.token_mint.to_account_info(),
            from: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.otc_swap.to_account_info(),
        };
        
        let burn_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            burn_accounts,
            signer,
        );
        
        token::burn(burn_ctx, burn_amount)?;
        
        emit!(TokensBurned {
            user: seller_key,
            amount: burn_amount,
            reason: "patron_exit_penalty".to_string(),
        });
    }
    
    // Transfer remaining tokens to buyer
    let transfer_to_buyer = Transfer {
        from: ctx.accounts.seller_token_account.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.otc_swap.to_account_info(),
    };
    
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_to_buyer,
        signer,
    );
    
    token::transfer(transfer_ctx, net_tokens_to_buyer)?;
    
    // Transfer SOL from buyer to seller
    **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? -= total_sol_payment;
    **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += total_sol_payment;
    
    emit!(SwapCompleted {
        seller: seller_key,
        buyer: buyer_key,
        otc_swap: otc_swap_key,
        token_amount: net_tokens_to_buyer,
        sol_payment: total_sol_payment,
        rebate_amount: 0,
    });
    
    Ok(())
}

// ========== TREASURY BUYBACK ==========

#[derive(Accounts)]
pub struct AcceptTreasuryBuyback<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        seeds = [b"reward_pool"],
        bump,
        constraint = reward_pool.admin == admin.key() @ SnakeError::Unauthorized,
    )]
    pub reward_pool: Account<'info, RewardPool>,
    
    /// CHECK: Validated through PDA seeds
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"otc_swap", seller.key().as_ref()],
        bump = otc_swap.bump,
        constraint = otc_swap.seller == seller.key() @ SnakeError::Unauthorized,
        constraint = otc_swap.is_active @ SnakeError::SwapInactive,
        constraint = otc_swap.swap_type == crate::state::SwapType::TreasuryBuyback @ SnakeError::InvalidSwapType,
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key(),
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = treasury_token_account.owner == reward_pool.treasury,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn accept_treasury_buyback(ctx: Context<AcceptTreasuryBuyback>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let _token_amount = ctx.accounts.otc_swap.token_amount;
    let sol_rate = ctx.accounts.otc_swap.sol_rate;
    let seller_key = ctx.accounts.seller.key();
    let otc_swap_key = ctx.accounts.otc_swap.key();
    let bump = ctx.accounts.otc_swap.bump;
    
    // Check if swap is expired
    require!(!ctx.accounts.otc_swap.is_expired(current_time), SnakeError::SwapExpired);
    
    // Calculate SOL payment
    let total_sol_payment = _token_amount
        .checked_mul(sol_rate)
        .ok_or(SnakeError::MathOverflow)?;
    
    // Verify treasury has sufficient SOL
    require!(
        ctx.accounts.treasury.to_account_info().lamports() >= total_sol_payment,
        SnakeError::InsufficientFunds
    );
    
    // Update swap state
    let swap = &mut ctx.accounts.otc_swap;
    swap.buyer = Some(ctx.accounts.reward_pool.treasury);
    swap.is_active = false;
    
    // Create PDA signer seeds
    let signer_seeds = &[
        b"otc_swap",
        seller_key.as_ref(),
        &[bump],
    ];
    let signer = &[&signer_seeds[..]];
    
    // Transfer tokens from seller to treasury
    let transfer_accounts = Transfer {
        from: ctx.accounts.seller_token_account.to_account_info(),
        to: ctx.accounts.treasury_token_account.to_account_info(),
        authority: ctx.accounts.otc_swap.to_account_info(),
    };
    
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer,
    );
    
    token::transfer(transfer_ctx, _token_amount)?;
    
    // Transfer SOL from treasury to seller
    **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? -= total_sol_payment;
    **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += total_sol_payment;
    
    emit!(SwapCompleted {
        seller: seller_key,
        buyer: ctx.accounts.reward_pool.treasury,
        otc_swap: otc_swap_key,
        token_amount: _token_amount,
        sol_payment: total_sol_payment,
        rebate_amount: 0,
    });
    
    Ok(())
}

// ========== FALLBACK TO TREASURY ==========

#[derive(Accounts)]
pub struct FallbackToTreasury<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        seeds = [b"reward_pool"],
        bump,
    )]
    pub reward_pool: Account<'info, RewardPool>,
    
    #[account(
        mut,
        seeds = [b"otc_swap", seller.key().as_ref()],
        bump = otc_swap.bump,
        constraint = otc_swap.seller == seller.key() @ SnakeError::Unauthorized,
        constraint = otc_swap.is_active @ SnakeError::SwapInactive,
        constraint = otc_swap.treasury_fallback @ SnakeError::TreasuryFallbackNotAllowed,
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key(),
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = treasury_token_account.owner == reward_pool.treasury,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn fallback_to_treasury(ctx: Context<FallbackToTreasury>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if swap is expired (required for fallback)
    require!(ctx.accounts.otc_swap.is_expired(current_time), SnakeError::SwapNotExpired);
    
    let _token_amount = ctx.accounts.otc_swap.token_amount;
    let sol_rate = ctx.accounts.otc_swap.sol_rate;
    let seller_key = ctx.accounts.seller.key();
    let otc_swap_key = ctx.accounts.otc_swap.key();
    let bump = ctx.accounts.otc_swap.bump;
    
    // Apply 10% discount for treasury buyback fallback
    let discounted_rate = sol_rate.saturating_mul(90).saturating_div(100);
    
    // Calculate burn amount if applicable
    let burn_amount = ctx.accounts.otc_swap.calculate_burn_amount();
    let net_tokens_to_treasury = ctx.accounts.otc_swap.calculate_net_tokens_after_burn();
    
    // Calculate SOL payment with discount
    let total_sol_payment = net_tokens_to_treasury
        .checked_mul(discounted_rate)
        .ok_or(SnakeError::MathOverflow)?;
    
    // Verify treasury has sufficient SOL
    require!(
        ctx.accounts.treasury.to_account_info().lamports() >= total_sol_payment,
        SnakeError::InsufficientFunds
    );
    
    // Update swap state
    let swap = &mut ctx.accounts.otc_swap;
    swap.buyer = Some(ctx.accounts.reward_pool.treasury);
    swap.is_active = false;
    
    // Create PDA signer seeds
    let signer_seeds = &[
        b"otc_swap",
        seller_key.as_ref(),
        &[bump],
    ];
    let signer = &[&signer_seeds[..]];
    
    // Burn tokens if penalty applies
    if burn_amount > 0 {
        let burn_accounts = Burn {
            mint: ctx.accounts.token_mint.to_account_info(),
            from: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.otc_swap.to_account_info(),
        };
        
        let burn_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            burn_accounts,
            signer,
        );
        
        token::burn(burn_ctx, burn_amount)?;
        
        emit!(TokensBurned {
            user: seller_key,
            amount: burn_amount,
            reason: "patron_exit_penalty".to_string(),
        });
    }
    
    // Transfer remaining tokens to treasury
    let transfer_accounts = Transfer {
        from: ctx.accounts.seller_token_account.to_account_info(),
        to: ctx.accounts.treasury_token_account.to_account_info(),
        authority: ctx.accounts.otc_swap.to_account_info(),
    };
    
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer,
    );
    
    token::transfer(transfer_ctx, net_tokens_to_treasury)?;
    
    // Transfer SOL from treasury to seller
    **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? -= total_sol_payment;
    **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += total_sol_payment;
    
    emit!(SwapCompleted {
        seller: seller_key,
        buyer: ctx.accounts.reward_pool.treasury,
        otc_swap: otc_swap_key,
        token_amount: net_tokens_to_treasury,
        sol_payment: total_sol_payment,
        rebate_amount: 0,
    });
    
    Ok(())
}