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
    let clock = Clock::get()?;

    let user_claim = &mut ctx.accounts.user_claim;
    let reward_pool = &mut ctx.accounts.reward_pool;

    // Initialize user claim state if it is not initialized
    if user_claim.initialized == false {
        user_claim.init(ctx.accounts.user.key());
    }

    // Check claim is available
    require!(
        reward_pool.minted_accum < LOWER_REWARDS_THREADHOLD,
        SnakeError::EndedClaim
    );

    // Enforce 24-hour cooldown (86400 seconds)
    require!(
        clock.unix_timestamp >= user_claim.last_claim_timestamp + 86400,
        SnakeError::CooldownNotPassed
    );

    // Update last claimed timestamp
    user_claim.last_claim_timestamp = clock.unix_timestamp;

    let (reward_amount, burn_amount, reward_level) =
        if reward_pool.minted_accum < HIGH_REWARDS_THREADHOLD {
            (625u64, 625u64, 1u8)
        } else if reward_pool.minted_accum < MEDIUM_REWARDS_THREADHOLD {
            (250u64, 250u64, 2u8)
        } else {
            (100u64, 100u64, 3u8)
        };

    let total_required = reward_amount
        .checked_add(burn_amount)
        .ok_or(SnakeError::ArithmeticOverflow)?;

    require!(
        ctx.accounts.treasury.amount >= total_required,
        SnakeError::InsufficientFundsInTreasury
    );

    reward_pool
        .minted_accum
        .checked_add(total_required)
        .ok_or(SnakeError::ArithmeticOverflow)?;

    // Transfer $SNK tokens from pool to buyer
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
        reward_amount * LAMPORTS_PER_SNK,
    )?;

    // Burn tokens from pool
    let token_burn_cpi_account = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.reward_pool.to_account_info(),
    };
    token::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_burn_cpi_account,
            &[&[REWARD_POOL_SEED, &[ctx.bumps.reward_pool]]],
        ),
        burn_amount * LAMPORTS_PER_SNK,
    )?;

    emit!(ClaimedReward {
        user: ctx.accounts.user.key(),
        reward_amount,
        burn_amount,
        reward_level,
    });

    Ok(())
}
