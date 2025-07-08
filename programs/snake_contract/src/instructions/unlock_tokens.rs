use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    state::{UserClaim, RewardPool},
    events::TokensUnlocked,
    errors::SnakeError,
};

#[derive(Accounts)]
pub struct UnlockTokens<'info> {
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
    
    /// Reward Pool PDA that holds the locked tokens
    #[account(
        seeds = [b"reward_pool"],
        bump,
    )]
    pub reward_pool_pda: Account<'info, RewardPool>,
    
    /// Treasury token account that holds the locked tokens
    #[account(
        mut,
        constraint = treasury_token_account.owner == reward_pool_pda.key(),
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn unlock_tokens(ctx: Context<UnlockTokens>) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let current_time = Clock::get()?.unix_timestamp;
    
    require!(user_claim.locked_amount > 0, SnakeError::NoTokensLocked);
    require!(user_claim.can_unlock(), SnakeError::LockPeriodNotCompleted);
    
    let unlock_amount = user_claim.locked_amount;
    
    // Create signer seeds for reward pool PDA
    let reward_pool_bump = ctx.bumps.reward_pool_pda;
    let reward_pool_signer_seeds: &[&[u8]] = &[
        b"reward_pool",
        &[reward_pool_bump],
    ];
    let reward_pool_signer = &[reward_pool_signer_seeds];
    
    // Transfer tokens back to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.treasury_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.reward_pool_pda.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        reward_pool_signer,
    );
    
    token::transfer(cpi_ctx, unlock_amount)?;
    
    // Reset lock information
    user_claim.locked_amount = 0;
    user_claim.lock_start_timestamp = 0;
    user_claim.lock_end_timestamp = 0;
    user_claim.lock_duration_months = 0;
    user_claim.last_yield_claim_timestamp = 0;
    
    emit!(TokensUnlocked {
        user: ctx.accounts.user.key(),
        amount: unlock_amount,
        timestamp: current_time,
    });
    
    Ok(())
}