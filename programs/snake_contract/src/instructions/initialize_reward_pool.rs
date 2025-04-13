use crate::{
    constants::{LAMPORTS_PER_SNK, REWARD_POOL_SEED, STAKE_AMOUNT},
    errors::SnakeError,
    state::RewardPool,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct InitializeRewardPool<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
        constraint = owner_ata.amount >= STAKE_AMOUNT * LAMPORTS_PER_SNK @ SnakeError::InsufficientFunds
    )]
    pub owner_ata: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = owner,
        space = 8 + RewardPool::INIT_SPACE,
        seeds=[REWARD_POOL_SEED],
        bump
    )]
    pub reward_pool: Account<'info, RewardPool>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = reward_pool,
        constraint = treasury.amount == 0
    )]
    pub treasury: Account<'info, TokenAccount>,

    // SNK token mint address
    pub mint: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_reward_pool(
    ctx: Context<InitializeRewardPool>,
    args: InitializeRewardPoolParams,
) -> Result<()> {
    let reward_pool = &mut ctx.accounts.reward_pool;
    reward_pool.init(
        ctx.accounts.owner.key(),
        args.admin,
        ctx.accounts.mint.key(),
        ctx.accounts.treasury.key(),
    );

    // Transfer 500,000,000 $SNK token to treasury
    let token_transfer_cpi_account = Transfer {
        from: ctx.accounts.owner_ata.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_transfer_cpi_account,
        ),
        STAKE_AMOUNT * LAMPORTS_PER_SNK,
    )?;

    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeRewardPoolParams {
    pub admin: Pubkey,
}
