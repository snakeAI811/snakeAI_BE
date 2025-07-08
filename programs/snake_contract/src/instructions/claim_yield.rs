use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, MintTo, Mint};
use crate::{
    state::{UserClaim, UserRole, RewardPool},
    events::YieldClaimed,
    errors::SnakeError,
};

#[derive(Accounts)]
pub struct ClaimYield<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    /// Reward Pool PDA that has minting authority
    #[account(
        seeds = [b"reward_pool"],
        bump,
    )]
    pub reward_pool_pda: Account<'info, RewardPool>,
    
    pub token_program: Program<'info, Token>,
}

pub fn claim_yield(ctx: Context<ClaimYield>) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let current_time = Clock::get()?.unix_timestamp;
    
    require!(user_claim.role == UserRole::Staker, SnakeError::Unauthorized);
    require!(user_claim.locked_amount > 0, SnakeError::NoTokensLocked);
    
    // Calculate yield amount
    let yield_amount = user_claim.calculate_yield()?;
    
    require!(yield_amount > 0, SnakeError::InsufficientFunds);
    
    // Create signer seeds for reward pool PDA
    let reward_pool_bump = ctx.bumps.reward_pool_pda;
    let reward_pool_signer_seeds: &[&[u8]] = &[
        b"reward_pool",
        &[reward_pool_bump],
    ];
    let reward_pool_signer = &[reward_pool_signer_seeds];
    
    // Mint yield tokens to user
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.reward_pool_pda.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        reward_pool_signer,
    );
    
    token::mint_to(cpi_ctx, yield_amount)?;
    
    // Update yield claim timestamp and total claimed
    user_claim.last_yield_claim_timestamp = current_time;
    user_claim.total_yield_claimed = user_claim.total_yield_claimed
        .checked_add(yield_amount)
        .ok_or(SnakeError::ArithmeticOverflow)?;
    
    emit!(YieldClaimed {
        user: ctx.accounts.user.key(),
        yield_amount,
        timestamp: current_time,
    });
    
    Ok(())
}