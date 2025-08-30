use anchor_lang::prelude::*;
use crate::state::RewardPool;
use crate::errors::SnakeError;
use crate::constants::REWARD_POOL_SEED;

#[derive(Accounts)]
pub struct StartTce<'info> {
    #[account(
        mut,
        seeds = [REWARD_POOL_SEED],
        bump,
        has_one = admin @ SnakeError::Unauthorized
    )]
    pub reward_pool: Account<'info, RewardPool>,
    
    /// Admin account that can start TCE
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

/// Start the Token Claim Event (TCE)
/// This allows users to claim their accumulated rewards on-chain
pub fn start_tce(ctx: Context<StartTce>) -> Result<()> {
    let reward_pool = &mut ctx.accounts.reward_pool;
    
    // Check if TCE has already started
    require!(!reward_pool.tce_started, SnakeError::TceAlreadyStarted);
    
    // Start the TCE
    reward_pool.tce_started = true;
    
    msg!("Token Claim Event (TCE) has been started by admin: {}", ctx.accounts.admin.key());
    
    Ok(())
}