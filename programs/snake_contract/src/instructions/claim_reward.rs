use crate::{
    constants::{
        HIGH_REWARDS_THREADHOLD, LAMPORTS_PER_SNK, LOWER_REWARDS_THREADHOLD,
        MEDIUM_REWARDS_THREADHOLD, REWARD_POOL_SEED, USER_CLAIM_SEED,
    },
    errors::SnakeError,
    events::ClaimedReward,
    state::{RewardPool, UserClaim},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Burn, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        has_one = treasury @ SnakeError::Unauthorized,
        has_one = mint @ SnakeError::Unauthorized,
        has_one = admin @ SnakeError::Unauthorized,
        seeds=[REWARD_POOL_SEED],
        bump
    )]
    pub reward_pool: Account<'info, RewardPool>,

    #[account(
        mut,
        address = reward_pool.treasury,
        constraint = treasury.amount != 0 @ SnakeError::InsufficientFundsInTreasury
    )]
    pub treasury: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserClaim::INIT_SPACE,
        seeds=[USER_CLAIM_SEED, user.key().as_ref()],
        bump
    )]
    pub user_claim: Account<'info, UserClaim>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_token_ata: Account<'info, TokenAccount>,

    // SNK token mint address
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {


    Ok(())
}
