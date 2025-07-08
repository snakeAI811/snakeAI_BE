use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    state::{UserClaim, UserRole, PatronStatus, RewardPool},
    events::TokensLocked,
    errors::SnakeError,
    constants::{STAKER_LOCK_DURATION_MONTHS, PATRON_LOCK_DURATION_MONTHS},
};

#[derive(Accounts)]
pub struct LockTokens<'info> {
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

pub fn lock_tokens(ctx: Context<LockTokens>, amount: u64, duration_months: u8) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let current_time = Clock::get()?.unix_timestamp;
    
    require!(amount > 0, SnakeError::CannotLockZeroTokens);
    require!(!user_claim.is_locked(), SnakeError::TokensLocked);
    
    // Validate duration based on user role
    let valid_duration = match user_claim.role {
        UserRole::Staker => duration_months == STAKER_LOCK_DURATION_MONTHS,
        UserRole::Patron => {
            // Only approved patrons can lock for 6 months
            require!(
                user_claim.patron_status == PatronStatus::Approved,
                SnakeError::OnlyApprovedPatrons
            );
            duration_months == PATRON_LOCK_DURATION_MONTHS
        },
        _ => false,
    };
    
    require!(valid_duration, SnakeError::InvalidLockDuration);
    
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