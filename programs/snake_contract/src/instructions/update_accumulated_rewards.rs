use anchor_lang::prelude::*;
use crate::state::{RewardPool, UserClaim};
use crate::errors::SnakeError;
use crate::constants::{REWARD_POOL_SEED, USER_CLAIM_SEED};

#[derive(Accounts)]
pub struct UpdateAccumulatedRewards<'info> {
    #[account(
        mut,
        seeds = [REWARD_POOL_SEED],
        bump,
        has_one = admin @ SnakeError::Unauthorized
    )]
    pub reward_pool: Account<'info, RewardPool>,
    
    #[account(
        mut,
        seeds = [USER_CLAIM_SEED, user.key().as_ref()],
        bump
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    /// User whose rewards are being updated
    /// CHECK: This is safe as we're only using it as a seed
    pub user: UncheckedAccount<'info>,
    
    /// Admin account that can update accumulated rewards
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

/// Update a user's accumulated rewards (Admin only)
/// This is used to sync off-chain rewards to on-chain during TCE
pub fn update_accumulated_rewards(
    ctx: Context<UpdateAccumulatedRewards>, 
    amount: u64
) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let reward_pool = &ctx.accounts.reward_pool;
    
    // Check if TCE has started (optional - you might want to allow this before TCE too)
    require!(reward_pool.tce_started, SnakeError::TceNotStarted);
    
    // Add the amount to accumulated rewards
    user_claim.accumulated_rewards = user_claim.accumulated_rewards
        .checked_add(amount)
        .ok_or(SnakeError::ArithmeticOverflow)?;
    
    msg!(
        "Updated accumulated rewards for user {}: added {} tokens, total now: {}", 
        ctx.accounts.user.key(), 
        amount, 
        user_claim.accumulated_rewards
    );
    
    Ok(())
}