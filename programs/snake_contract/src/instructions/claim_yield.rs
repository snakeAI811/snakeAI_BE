use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};
use crate::{
    state::{UserClaim, UserRole, RewardPool, UserStakingHistory, GlobalStakingStats, StakingHistoryEntry, StakingAction},
    events::YieldClaimed,
    errors::SnakeError,
    constants::{
        USER_CLAIM_SEED,
        REWARD_POOL_SEED,
        LAMPORTS_PER_SNK,
        YIELD_CLAIM_COOLDOWN_SECONDS,
        USER_STAKING_HISTORY_SEED,
        GLOBAL_STAKING_STATS_SEED
    },
    utils::{ValidationUtils, CalculationUtils}
};

#[derive(Accounts)]
pub struct ClaimYield<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [USER_CLAIM_SEED, user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    /// Reward Pool PDA
    #[account(
        seeds = [REWARD_POOL_SEED],
        bump,
    )]
    pub reward_pool_pda: Account<'info, RewardPool>,
    
    /// Treasury token account owned by reward pool PDA
    #[account(
        mut,
        constraint = treasury.owner == reward_pool_pda.key(),
        constraint = treasury.mint == mint.key(),
    )]
    pub treasury: Account<'info, TokenAccount>,
    
    /// User staking history PDA
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserStakingHistory::INIT_SPACE,
        seeds = [USER_STAKING_HISTORY_SEED, user.key().as_ref()],
        bump,
    )]
    pub user_staking_history: Account<'info, UserStakingHistory>,
    
    /// Global staking stats PDA
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + GlobalStakingStats::INIT_SPACE,
        seeds = [GLOBAL_STAKING_STATS_SEED],
        bump,
    )]
    pub global_staking_stats: Account<'info, GlobalStakingStats>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn claim_yield(ctx: Context<ClaimYield>) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate user role and locked amount
    ValidationUtils::validate_user_role(user_claim, &[UserRole::Staker, UserRole::Patron])?;
    require!(user_claim.locked_amount > 0, SnakeError::NoTokensLocked);
    
    // Validate cooldown period
    ValidationUtils::validate_cooldown(
        user_claim.last_yield_claim_timestamp,
        YIELD_CLAIM_COOLDOWN_SECONDS,
    )?;
    
    // Calculate yield amount using utility function
    let yield_amount = CalculationUtils::calculate_yield(user_claim, current_time);
    
    require!(yield_amount > 0, SnakeError::InsufficientFunds);
    
    // Create signer seeds for reward pool PDA
    let reward_pool_bump = ctx.bumps.reward_pool_pda;
    let reward_pool_signer_seeds: &[&[u8]] = &[
        REWARD_POOL_SEED,
        &[reward_pool_bump],
    ];
    let reward_pool_signer = &[reward_pool_signer_seeds];
    
    // Transfer yield tokens from treasury to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.treasury.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.reward_pool_pda.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        reward_pool_signer,
    );
    
    token::transfer(cpi_ctx, yield_amount)?;
    
    // Update yield claim timestamp and total claimed
    user_claim.last_yield_claim_timestamp = current_time;
    user_claim.total_yield_claimed = CalculationUtils::safe_add(
        user_claim.total_yield_claimed,
        yield_amount
    )?;
    
    // Initialize history accounts if needed
    let user_history = &mut ctx.accounts.user_staking_history;
    if !user_history.initialized {
        user_history.init(ctx.accounts.user.key());
    }
    
    let global_stats = &mut ctx.accounts.global_staking_stats;
    if !global_stats.initialized {
        global_stats.init();
    }
    
    // Add history entry for yield claim
    let history_entry = StakingHistoryEntry {
        action: StakingAction::YieldClaim,
        amount: 0, // No amount change for yield claims
        timestamp: current_time,
        role: user_claim.role.clone(),
        lock_duration_months: user_claim.lock_duration_months,
        yield_amount,
        additional_data: format!("Yield claim: {} tokens", yield_amount / LAMPORTS_PER_SNK),
    };
    
    user_history.add_entry(history_entry)?;
    global_stats.add_yield_distributed(yield_amount)?;
    
    emit!(YieldClaimed {
        user: ctx.accounts.user.key(),
        yield_amount: yield_amount,
        timestamp: current_time,
    });
    
    Ok(())
}