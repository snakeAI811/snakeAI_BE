use crate::{errors::SnakeError, state::RewardPool};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateRewardPool<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        has_one = owner @ SnakeError::Unauthorized
    )]
    pub reward_pool: Account<'info, RewardPool>,
}

pub fn update_reward_pool(
    ctx: Context<UpdateRewardPool>,
    args: UpdateRewardPoolParams,
) -> Result<()> {
    let reward_pool = &mut ctx.accounts.reward_pool;
    reward_pool.admin = args.admin;

    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateRewardPoolParams {
    pub admin: Pubkey,
}
