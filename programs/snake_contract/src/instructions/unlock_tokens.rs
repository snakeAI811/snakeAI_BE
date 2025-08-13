use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    state::{UserClaim, RewardPool, UserStakingHistory, GlobalStakingStats, StakingHistoryEntry, StakingAction},
    events::TokensUnlocked,
    errors::SnakeError,
    constants::{
        USER_CLAIM_SEED, 
        REWARD_POOL_SEED,
        USER_STAKING_HISTORY_SEED,
        GLOBAL_STAKING_STATS_SEED,
        LAMPORTS_PER_SNK
    }
};

#[derive(Accounts)]
pub struct UnlockTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [USER_CLAIM_SEED, user.key().as_ref()],
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
        seeds = [REWARD_POOL_SEED],
        bump,
    )]
    pub reward_pool_pda: Account<'info, RewardPool>,
    
    /// Treasury token account that holds the locked tokens
    #[account(
        mut,
        constraint = treasury_token_account.owner == reward_pool_pda.key(),
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    /// User staking history PDA
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserStakingHistory::INIT_SPACE,
        seeds = [USER_STAKING_HISTORY_SEED, user.key().as_ref()],
        bump,
    )]
    pub user_staking_history: Account<'info, UserStakingHistory>,
    
    /// Global staking stats PDA
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + GlobalStakingStats::INIT_SPACE,
        seeds = [GLOBAL_STAKING_STATS_SEED],
        bump,
    )]
    pub global_staking_stats: Account<'info, GlobalStakingStats>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
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
        REWARD_POOL_SEED,
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
    
    // Initialize history accounts if needed
    let user_history = &mut ctx.accounts.user_staking_history;
    if !user_history.initialized {
        user_history.init(ctx.accounts.user.key());
    }
    
    let global_stats = &mut ctx.accounts.global_staking_stats;
    if !global_stats.initialized {
        global_stats.init();
    }
    
    // Add history entry for token unlock
    let history_entry = StakingHistoryEntry {
        action: StakingAction::Unlock,
        amount: unlock_amount,
        timestamp: current_time,
        role: user_claim.role.clone(),
        lock_duration_months: 0, // No longer locked
        yield_amount: 0,
        additional_data: format!("Unlocked {} tokens", unlock_amount / LAMPORTS_PER_SNK),
    };
    
    user_history.add_entry(history_entry)?;
    global_stats.update_locked_amount(-(unlock_amount as i64))?;
    
    emit!(TokensUnlocked {
        user: ctx.accounts.user.key(),
        amount: unlock_amount,
        timestamp: current_time,
    });
    
    Ok(())
}