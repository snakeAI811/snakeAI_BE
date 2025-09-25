use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};
use crate::{
    state::{UserClaim, UserRole, OtcSwap, SwapType},
    errors::SnakeError,
    utils::{ValidationUtils, CalculationUtils},
    constants::*,
};
use super::{
    validation::OtcSwapValidation,
    events::OtcSwapEvents,
    core::OtcSwapCore,
    tracking::{OtcSwapTracker, OtcSwapTracking},
    deflationary::{DeflationaryMechanics, DailyVolumeTracker},
};

// ========== INSTRUCTION STRUCTURES ==========

#[derive(Accounts)]
pub struct InitiateOtcSwap<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    // Auto-create on first call
    #[account(
        init_if_needed,
        payer = seller,
        space = 8 + UserClaim::INIT_SPACE,
        seeds = [b"user_claim", seller.key().as_ref()],
        bump,
    )]
    pub seller_claim: Account<'info, UserClaim>,

    #[account(
        init_if_needed,
        payer = seller,
        space = 8 + OtcSwap::INIT_SPACE,
        seeds = [b"otc_swap", seller.key().as_ref()],
        bump,
    )]
    pub otc_swap: Account<'info, OtcSwap>,

    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key(),
        // amount >= 0 constraint is redundant; TokenAccount.amount is u64.
    )]
    pub seller_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptOtcSwap<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + UserClaim::INIT_SPACE,
        seeds = [b"user_claim", buyer.key().as_ref()],
        bump,
    )]
    pub buyer_claim: Account<'info, UserClaim>,

    #[account(
        mut,
        seeds = [b"otc_swap", otc_swap.seller.as_ref()],
        bump,
        constraint = otc_swap.is_active @ SnakeError::SwapInactive,
    )]
    pub otc_swap: Account<'info, OtcSwap>,

    #[account(
        mut,
        seeds = [b"user_claim", otc_swap.seller.as_ref()],
        bump,
        constraint = seller_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub seller_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        constraint = buyer_token_account.owner == buyer.key(),
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == otc_swap.seller,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = treasury_account.owner == reward_pool.key(),
    )]
    pub treasury_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [b"reward_pool"],
        bump,
    )]
    pub reward_pool: Account<'info, crate::state::RewardPool>,
    
    #[account(
        mut,
        constraint = mint.key() == seller_token_account.mint,
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + DailyVolumeTracker::INIT_SPACE,
        seeds = [b"daily_volume_tracker"],
        bump,
    )]
    pub daily_volume_tracker: Account<'info, DailyVolumeTracker>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelOtcSwap<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"otc_swap", seller.key().as_ref()],
        bump,
        constraint = otc_swap.seller == seller.key() @ SnakeError::Unauthorized,
        // allow cancel during cooldown; active check moved to handler
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    pub system_program: Program<'info, System>,
}

// ========== INSTRUCTION IMPLEMENTATIONS ==========

pub fn initiate_swap(
    ctx: Context<InitiateOtcSwap>,
    token_amount: u64,
    sol_rate: u64,
    buyer_rebate: u64,
    swap_type: SwapType,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;

    // If just created, mark claim initialized
    if !ctx.accounts.seller_claim.initialized {
        // FIXED: Only pass the user key, not the bump
        ctx.accounts.seller_claim.init(ctx.accounts.seller.key());
    }

    OtcSwapValidation::validate_swap_params(token_amount, sol_rate, buyer_rebate)?;
    OtcSwapValidation::validate_seller_eligibility(&ctx.accounts.seller_claim, &swap_type, current_time)?;

    OtcSwapCore::initialize_swap(
        &mut ctx.accounts.otc_swap,
        ctx.accounts.seller.key(),
        token_amount,
        sol_rate,
        buyer_rebate,
        swap_type.clone(),
        current_time,
        ctx.bumps.otc_swap, // FIXED: Direct field access instead of .get()
    )?;

    OtcSwapEvents::emit_swap_initiated(
        ctx.accounts.seller.key(),
        swap_type,
        token_amount,
        sol_rate,
        ctx.accounts.otc_swap.key(),
        buyer_rebate,
    );
    Ok(())
}

pub fn accept_swap(
    ctx: Context<AcceptOtcSwap>,
    buyer_rebate: u64,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    // Initialize buyer claim if needed
    if !ctx.accounts.buyer_claim.initialized {
        // FIXED: Only pass the user key, not the bump
        ctx.accounts.buyer_claim.init(ctx.accounts.buyer.key());
    }
    
    // Validate buyer eligibility
    OtcSwapValidation::validate_buyer_eligibility(
        &ctx.accounts.buyer_claim,
        &ctx.accounts.otc_swap.swap_type,
    )?;
    
    // Validate swap is active
    OtcSwapValidation::validate_swap_active(
        ctx.accounts.otc_swap.is_active,
        false, // is_completed
        false, // is_cancelled
        None,  // expiration_timestamp
    )?;
    
    // Execute swap with deflationary mechanics
    let token_amount = ctx.accounts.otc_swap.token_amount;
    OtcSwapCore::execute_otc_swap_with_deflationary(
        &mut ctx.accounts.otc_swap,
        &mut ctx.accounts.seller_claim,
        &ctx.accounts.buyer_claim,
        token_amount,
        &ctx.accounts.seller_token_account,
        &ctx.accounts.buyer_token_account,
        &ctx.accounts.treasury_account,
        &ctx.accounts.mint,
        &ctx.accounts.reward_pool.to_account_info(),
        &ctx.accounts.token_program,
        // FIXED: Use direct field access for reward_pool bump
        &[&[b"reward_pool", &[ctx.bumps.reward_pool]]],
        &mut ctx.accounts.daily_volume_tracker,
        current_time,
    )?;
    
    // Emit completion event
    OtcSwapEvents::emit_swap_completed(
        ctx.accounts.otc_swap.seller,
        ctx.accounts.buyer.key(),
        ctx.accounts.otc_swap.token_amount,
        ctx.accounts.otc_swap.sol_rate,
        ctx.accounts.otc_swap.key(),
        buyer_rebate,
    );
    
    Ok(())
}

pub fn cancel_swap(ctx: Context<CancelOtcSwap>) -> Result<()> {
    let otc = &mut ctx.accounts.otc_swap;

    // Cannot cancel after acceptance/completion
    require!(otc.buyer.is_none(), SnakeError::SwapAlreadyAccepted);

    if !otc.is_active {
        // Already inactive (cooldown and not activated yet, or already canceled)
        // Make cancel idempotent for better UX across all swap types
        return Ok(());
    }

    // Cancel active swap
    otc.is_active = false;

    // Emit cancellation event once upon actual state change
    OtcSwapEvents::emit_swap_cancelled(ctx.accounts.seller.key(), otc.key());

    Ok(())
}

// ========== TRACKING FUNCTIONS ==========

#[derive(Accounts)]
pub struct TrackOtcSwap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + OtcSwapTracker::INIT_SPACE,
        seeds = [b"otc_tracker", user.key().as_ref()],
        bump,
    )]
    pub otc_tracker: Account<'info, OtcSwapTracker>,
    
    pub system_program: Program<'info, System>,
}

pub fn track_swap_operation(
    ctx: Context<TrackOtcSwap>,
    amount: u64,
    is_sale: bool,
) -> Result<()> {
    OtcSwapTracking::track_swap_operation(
        &mut ctx.accounts.otc_tracker,
        &ctx.accounts.user_claim,
        amount,
        is_sale,
    )
}

#[derive(Accounts)]
pub struct ApplyBurnPenalty<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"otc_tracker", user.key().as_ref()],
        bump,
    )]
    pub otc_tracker: Account<'info, OtcSwapTracker>,
}

pub fn apply_burn_penalty(
    ctx: Context<ApplyBurnPenalty>,
    burn_amount: u64,
) -> Result<()> {
    OtcSwapTracking::apply_burn_penalty(
        &mut ctx.accounts.otc_tracker,
        &mut ctx.accounts.user_claim,
        burn_amount,
    )
}

#[derive(Accounts)]
pub struct RevokeDAOEligibility<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"otc_tracker", user.key().as_ref()],
        bump,
    )]
    pub otc_tracker: Account<'info, OtcSwapTracker>,
}

pub fn revoke_dao_eligibility(ctx: Context<RevokeDAOEligibility>) -> Result<()> {
    OtcSwapTracking::revoke_dao_eligibility(
        &mut ctx.accounts.otc_tracker,
        &mut ctx.accounts.user_claim,
    )
}

#[derive(Accounts)]
pub struct GetSwapTrackerStats<'info> {
    #[account(
        seeds = [b"otc_tracker", user.key().as_ref()],
        bump,
    )]
    pub otc_tracker: Account<'info, OtcSwapTracker>,
    
    /// CHECK: This is just for reading stats
    pub user: UncheckedAccount<'info>,
}

pub fn get_swap_tracker_stats(ctx: Context<GetSwapTrackerStats>) -> Result<(u64, u64, i64, bool, bool, bool)> {
    let tracker = &ctx.accounts.otc_tracker;
    Ok((
        tracker.total_swaps,
        tracker.total_volume,
        tracker.last_swap_timestamp,
        tracker.exit_tracked,
        tracker.burn_penalty_applied,
        tracker.dao_eligibility_revoked,
    ))
}
