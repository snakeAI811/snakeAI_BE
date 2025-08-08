use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    state::{UserClaim, UserRole, PatronStatus, RewardPool},
    events::TokensLocked,
    errors::SnakeError,
    constants::{
        STAKER_LOCK_DURATION_MONTHS, 
        PATRON_LOCK_DURATION_MONTHS,
        PATRON_MIN_TOKEN_AMOUNT,
        PATRON_MIN_WALLET_AGE_DAYS,
        PATRON_MIN_STAKING_MONTHS,
        STAKER_MIN_STAKING_MONTHS,
        USER_CLAIM_SEED
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
        seeds = [b"reward_pool"],
        bump,
    )]
    pub reward_pool_pda: Account<'info, RewardPool>,
    
    /// Treasury token account that will receive the locked tokens
    #[account(
        mut,
        constraint = treasury_token_account.owner == reward_pool_pda.key(),
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
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
    
    // Validate duration and eligibility based on user role
    // let valid_duration = match user_claim.role {
    //     UserRole::Staker => {
    //         // Check if user has staking history (3+ months)
    //         require!(
    //             has_sufficient_staking_history(user_claim, STAKER_MIN_STAKING_MONTHS),
    //             SnakeError::InsufficientStakingHistory
    //         );
    //         duration_months == STAKER_LOCK_DURATION_MONTHS
    //     },
    //     UserRole::Patron => {
    //         // Validate patron eligibility criteria
    //         require!(
    //             validate_patron_eligibility(user_claim, amount),
    //             SnakeError::PatronEligibilityNotMet
    //         );
            
    //         // Only approved patrons can lock for 6 months
    //         require!(
    //             user_claim.patron_status == PatronStatus::Approved,
    //             SnakeError::OnlyApprovedPatrons
    //         );
    //         duration_months == PATRON_LOCK_DURATION_MONTHS
    //     },
    //     _ => false,
    // };
    
    require!(duration_months >= 3, SnakeError::InvalidLockDuration);
    
    // Calculate lock end time
    let seconds_in_month = 30 * 24 * 60 * 60; // Approximate
    let lock_duration_seconds = (duration_months as i64) * seconds_in_month;
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
    
    // Update user claim with lock information
    user_claim.locked_amount = amount;
    user_claim.lock_start_timestamp = current_time;
    user_claim.lock_end_timestamp = lock_end_time;
    user_claim.lock_duration_months = duration_months;
    user_claim.last_yield_claim_timestamp = current_time;
    
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