
use crate::{
    constants::{REWARD_POOL_SEED, USER_CLAIM_SEED},
    errors::SnakeError,
    state::{RewardPool, UserClaim},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct BatchClaim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        has_one = treasury @ SnakeError::Unauthorized,
        has_one = mint @ SnakeError::Unauthorized,
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
        mut,
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

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn batch_claim(ctx: Context<BatchClaim>) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let reward_pool = &mut ctx.accounts.reward_pool;

    // Check if TCE has started
    require!(reward_pool.tce_started, SnakeError::TceNotStarted);

    require!(user_claim.accumulated_rewards > 0, SnakeError::NoRewardsToClaim);

    let reward_amount = user_claim.accumulated_rewards;

    require!(
        ctx.accounts.treasury.amount >= reward_amount,
        SnakeError::InsufficientFundsInTreasury
    );

    reward_pool.minted_accum = reward_pool.minted_accum.checked_add(reward_amount).unwrap();

    let token_transfer_cpi_account = Transfer {
        from: ctx.accounts.treasury.to_account_info(),
        to: ctx.accounts.user_token_ata.to_account_info(),
        authority: ctx.accounts.reward_pool.to_account_info(),
    };
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_transfer_cpi_account,
            &[&[REWARD_POOL_SEED, &[ctx.bumps.reward_pool]]],
        ),
        reward_amount,
    )?;

    user_claim.accumulated_rewards = 0;

    Ok(())
}