use crate::{
    constants::{LAMPORTS_PER_SNK, REWARD_POOL_SEED},
    errors::SnakeError,
    state::RewardPool,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        has_one = owner @ SnakeError::Unauthorized,
        seeds=[REWARD_POOL_SEED],
        bump
    )]
    pub reward_pool: Account<'info, RewardPool>,

    #[account(
        mut,
        address = reward_pool.treasury,
        associated_token::mint = mint,
        associated_token::authority = reward_pool,
        constraint = treasury.amount != 0 @ SnakeError::InsufficientFundsInTreasury
    )]
    pub treasury: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub owner_token_ata: Account<'info, TokenAccount>,

    // SNK token mint address
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw(ctx: Context<Withdraw>, args: WithdrawParams) -> Result<()> {
    require!(
        ctx.accounts.treasury.amount >= args.amount,
        SnakeError::InsufficientFundsInTreasury
    );

    // Transfer $SNK tokens from pool to owner
    let token_transfer_cpi_account = Transfer {
        from: ctx.accounts.treasury.to_account_info(),
        to: ctx.accounts.owner_token_ata.to_account_info(),
        authority: ctx.accounts.reward_pool.to_account_info(),
    };
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_transfer_cpi_account,
            &[&[REWARD_POOL_SEED, &[ctx.bumps.reward_pool]]],
        ),
        args.amount * LAMPORTS_PER_SNK,
    )?;

    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawParams {
    pub amount: u64,
}
