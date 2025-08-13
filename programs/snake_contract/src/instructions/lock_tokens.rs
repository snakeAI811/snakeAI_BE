use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    state::{UserClaim, UserRole, PatronStatus, RewardPool, UserStakingHistory, GlobalStakingStats, StakingHistoryEntry, StakingAction},
    events::TokensLocked,
    errors::SnakeError,
    constants::{
        STAKER_LOCK_DURATION_MONTHS, 
        PATRON_LOCK_DURATION_MONTHS,
        PATRON_MIN_TOKEN_AMOUNT,
        PATRON_MIN_WALLET_AGE_DAYS,
        PATRON_MIN_STAKING_MONTHS,
        STAKER_MIN_STAKING_MONTHS,
        USER_CLAIM_SEED,
        SECONDS_PER_MONTH,
        REWARD_POOL_SEED,
        USER_STAKING_HISTORY_SEED,
        GLOBAL_STAKING_STATS_SEED,
        LAMPORTS_PER_SNK
    },
};  

#[derive(Accounts)]
pub struct LockTokens<'info> {
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
    
    /// Reward Pool PDA that will hold the locked tokens
    #[account(
        seeds = [REWARD_POOL_SEED],
        bump,
    )]
    pub reward_pool_pda: Account<'info, RewardPool>,
    
    /// Treasury token account that will receive the locked tokens
    #[account(
        mut,
        constraint = treasury_token_account.owner == reward_pool_pda.key(),
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
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

/// Helper function to check if user has sufficient staking history
fn has_sufficient_staking_history(user_claim: &UserClaim, required_months: u8) -> bool {
    if user_claim.lock_start_timestamp == 0 {
        return false; // No staking history
    }
    
    let current_time = Clock::get().unwrap().unix_timestamp;
    let staking_duration_seconds = current_time.saturating_sub(user_claim.lock_start_timestamp);
    let required_seconds = (required_months as i64) * 30 * 24 * 60 * 60; // Approximate months to seconds
    
    staking_duration_seconds >= required_seconds
}

/// Helper function to validate patron eligibility
fn validate_patron_eligibility(user_claim: &UserClaim, token_amount: u64) -> bool {
    // Check token amount (>=250k)
    if token_amount < PATRON_MIN_TOKEN_AMOUNT {
        return false;
    }
    
    // Check minimum mining threshold (> 0)
    if user_claim.total_mined_phase1 == 0 {
        return false;
    }
    
    // Check wallet age (>= 30 days)
    if user_claim.wallet_age_days < PATRON_MIN_WALLET_AGE_DAYS {
        return false;
    }
    
    // Check staking history (6 months)
    if !has_sufficient_staking_history(user_claim, PATRON_MIN_STAKING_MONTHS) {
        return false;
    }
    
    true
}

pub fn lock_tokens(ctx: Context<LockTokens>, amount: u64, duration_months: u8) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let current_time = Clock::get()?.unix_timestamp;
    
    require!(amount > 0, SnakeError::CannotLockZeroTokens);
    require!(!user_claim.is_locked(), SnakeError::TokensLocked);
    
    // Both stakers and patrons need minimum 5000 tokens
    require!(amount >= 5000, SnakeError::InsufficientFunds);
    
    // Both can lock for 3 or 6 months
    require!(
        duration_months == STAKER_LOCK_DURATION_MONTHS || duration_months == PATRON_LOCK_DURATION_MONTHS,
        SnakeError::InvalidLockDuration
    );
    
    // Validate user has valid role (either Staker or Patron)
    require!(
        user_claim.role == UserRole::Staker || user_claim.role == UserRole::Patron,
        SnakeError::InvalidRole
    );
    
    // Calculate lock end time
    let lock_duration_seconds = (duration_months as i64) * SECONDS_PER_MONTH;
    let lock_end_time = current_time + lock_duration_seconds;
    
    // Transfer tokens to treasury (locked)
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.treasury_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    
    token::transfer(cpi_ctx, amount)?;
    
    // Update user claim with lock information (DON'T change role)
    user_claim.locked_amount = amount;
    user_claim.lock_start_timestamp = current_time;
    user_claim.lock_end_timestamp = lock_end_time;
    user_claim.lock_duration_months = duration_months;
    user_claim.last_yield_claim_timestamp = current_time;
    
    // Initialize history accounts if needed
    let user_history = &mut ctx.accounts.user_staking_history;
    if !user_history.initialized {
        user_history.init(ctx.accounts.user.key());
    }
    
    let global_stats = &mut ctx.accounts.global_staking_stats;
    if !global_stats.initialized {
        global_stats.init();
    }
    
    // Add history entry for token lock
    let history_entry = StakingHistoryEntry {
        action: StakingAction::Lock,
        amount,
        timestamp: current_time,
        role: user_claim.role.clone(),
        lock_duration_months: duration_months,
        yield_amount: 0,
        additional_data: format!("Locked {} tokens for {} months", amount / LAMPORTS_PER_SNK, duration_months),
    };
    
    user_history.add_entry(history_entry)?;
    global_stats.update_locked_amount(amount as i64)?;
    
    emit!(TokensLocked {
        user: ctx.accounts.user.key(),
        amount,
        duration_months,
        lock_start: current_time,
        lock_end: lock_end_time,
        role: user_claim.role.clone(),
    });
    
    Ok(())
}