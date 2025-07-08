use anchor_lang::prelude::*;
use crate::{
    state::{UserClaim, RewardPool},
    errors::SnakeError,
};

/// Update user statistics for patron qualification
/// This allows admin to set mining amounts, wallet age, and community scores

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateUserStatsParams {
    pub phase1_mined: Option<u64>,
    pub wallet_age_days: Option<u32>,
    pub community_score: Option<u32>,
    pub phase2_mining_completed: Option<bool>,
}

#[derive(Accounts)]
pub struct UpdateUserStats<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        seeds = [b"reward_pool"],
        bump,
        constraint = reward_pool.admin == admin.key() @ SnakeError::Unauthorized,
    )]
    pub reward_pool: Account<'info, RewardPool>,
    
    /// CHECK: User whose stats are being updated
    pub user: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub user_claim: Account<'info, UserClaim>,
}

pub fn update_user_stats(
    ctx: Context<UpdateUserStats>,
    params: UpdateUserStatsParams,
) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    
    // Update Phase 1 mining amount
    if let Some(amount) = params.phase1_mined {
        user_claim.total_mined_phase1 = amount;
    }
    
    // Update wallet age (for patron qualification)
    if let Some(age) = params.wallet_age_days {
        user_claim.wallet_age_days = age;
    }
    
    // Update community score (0-100, capped at 30 for scoring)
    if let Some(score) = params.community_score {
        user_claim.community_score = score.min(100);
    }
    
    // Update Phase 2 mining status
    if let Some(completed) = params.phase2_mining_completed {
        user_claim.mined_in_phase2 = completed;
    }
    
    // Recalculate qualification score
    user_claim.calculate_patron_qualification_score();
    
    Ok(())
}

/// Batch update multiple users (for efficiency)
#[derive(Accounts)]
pub struct BatchUpdateUserStats<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        seeds = [b"reward_pool"],
        bump,
        constraint = reward_pool.admin == admin.key() @ SnakeError::Unauthorized,
    )]
    pub reward_pool: Account<'info, RewardPool>,
}

pub fn batch_update_user_stats(
    ctx: Context<BatchUpdateUserStats>,
    updates: Vec<(Pubkey, UpdateUserStatsParams)>,
) -> Result<()> {
    // In a full implementation, this would iterate through multiple user accounts
    // For now, this is a placeholder for batch operations
    require!(updates.len() <= 10, SnakeError::InvalidAmount); // Limit batch size
    
    // This would need to be implemented with remaining_accounts
    // or multiple instruction calls for now
    
    Ok(())
}