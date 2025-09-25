
use crate::{
    constants::{REWARD_POOL_SEED, USER_CLAIM_SEED},
    errors::SnakeError,
    state::{RewardPool, UserClaim},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct LogClaim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[REWARD_POOL_SEED],
        bump
    )]
    pub reward_pool: Account<'info, RewardPool>,

    #[account(
        mut,
        seeds=[USER_CLAIM_SEED, user.key().as_ref()],
        bump
    )]
    pub user_claim: Account<'info, UserClaim>,

    pub system_program: Program<'info, System>,
}

fn get_reward_burn_amount(tweet_count: i64) -> (u64, u64) {
    match tweet_count {
        1..=200_000 => (375, 375),
        200_001..=500_000 => (150, 150),
        500_001..=1_000_000 => (60, 60),
        1_000_001..=3_500_000 => (40, 40),
        _ => (0, 0), // No rewards after 3.5M
    }
}

pub fn log_claim(ctx: Context<LogClaim>) -> Result<()> {
    let clock = Clock::get()?;
    let user_claim = &mut ctx.accounts.user_claim;
    let reward_pool = &mut ctx.accounts.reward_pool;

    // Enforce 25min cooldown (1500 seconds)
    require!(
        clock.unix_timestamp >= user_claim.last_claim_timestamp + 1500,
        SnakeError::CooldownNotPassed
    );

    // Update last claimed timestamp
    user_claim.last_claim_timestamp = clock.unix_timestamp;

    // Use reward schedule logic based on global progress
    let tweet_count = reward_pool.minted_accum as i64; // 👈 global counter proxy
    let (reward_amount, burn_amount) = get_reward_burn_amount(tweet_count);

    // If rewards are finished, prevent claim
    require!(reward_amount > 0, SnakeError::InvalidAmount);

    // Accumulate user rewards
    user_claim.accumulated_rewards = user_claim
        .accumulated_rewards
        .checked_add(reward_amount)
        .ok_or(SnakeError::InvalidAmount)?;

    // Track global minted accumulation
    reward_pool.minted_accum = reward_pool
        .minted_accum
        .checked_add(reward_amount)
        .ok_or(SnakeError::InvalidAmount)?;

    // (Optional) you can log burn_amount or store it for future
    msg!(
        "User {} claimed {} tokens ({} burned)",
        ctx.accounts.user.key(),
        reward_amount,
        burn_amount
    );

    Ok(())
}