use anchor_lang::prelude::*;
use crate::{
    state::{UserStakingHistory, GlobalStakingStats},
    constants::{USER_STAKING_HISTORY_SEED, GLOBAL_STAKING_STATS_SEED},
    errors::SnakeError,
};

#[derive(Accounts)]
pub struct GetStakingHistory<'info> {
    pub user: Signer<'info>,
    
    /// User staking history PDA
    #[account(
        seeds = [USER_STAKING_HISTORY_SEED, user.key().as_ref()],
        bump,
    )]
    pub user_staking_history: Account<'info, UserStakingHistory>,
}

#[derive(Accounts)]
pub struct GetGlobalStats<'info> {
    /// Global staking stats PDA
    #[account(
        seeds = [GLOBAL_STAKING_STATS_SEED],
        bump,
    )]
    pub global_staking_stats: Account<'info, GlobalStakingStats>,
}

#[derive(Accounts)]
pub struct InitializeStakingHistory<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// User staking history PDA
    #[account(
        init,
        payer = user,
        space = 8 + UserStakingHistory::INIT_SPACE,
        seeds = [USER_STAKING_HISTORY_SEED, user.key().as_ref()],
        bump,
    )]
    pub user_staking_history: Account<'info, UserStakingHistory>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeGlobalStats<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// Global staking stats PDA
    #[account(
        init,
        payer = admin,
        space = 8 + GlobalStakingStats::INIT_SPACE,
        seeds = [GLOBAL_STAKING_STATS_SEED],
        bump,
    )]
    pub global_staking_stats: Account<'info, GlobalStakingStats>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_staking_history(ctx: Context<InitializeStakingHistory>) -> Result<()> {
    let user_history = &mut ctx.accounts.user_staking_history;
    user_history.init(ctx.accounts.user.key());
    Ok(())
}

pub fn initialize_global_stats(ctx: Context<InitializeGlobalStats>) -> Result<()> {
    let global_stats = &mut ctx.accounts.global_staking_stats;
    global_stats.init();
    Ok(())
}

// View functions (these would typically be called via RPC, not as instructions)
pub fn get_user_staking_summary(ctx: Context<GetStakingHistory>) -> Result<()> {
    let history = &ctx.accounts.user_staking_history;
    
    require!(history.initialized, SnakeError::Unauthorized);
    
    msg!("=== User Staking Summary ===");
    msg!("User: {}", history.user);
    msg!("Total Entries: {}", history.total_entries);
    msg!("Total Locked: {}", history.total_locked);
    msg!("Total Unlocked: {}", history.total_unlocked);
    msg!("Total Yield Claimed: {}", history.total_yield_claimed);
    msg!("First Stake: {}", history.first_stake_timestamp);
    msg!("Last Activity: {}", history.last_activity_timestamp);
    
    let current_time = Clock::get()?.unix_timestamp;
    let staking_days = history.get_staking_duration_days(current_time);
    msg!("Staking Duration: {} days", staking_days);
    
    // Show recent entries
    let recent_entries = history.get_recent_entries(10);
    msg!("=== Recent Activity (Last 10) ===");
    for (i, entry) in recent_entries.iter().enumerate() {
        msg!("{}. Action: {:?}, Amount: {}, Yield: {}, Time: {}", 
             i + 1, entry.action, entry.amount, entry.yield_amount, entry.timestamp);
    }
    
    Ok(())
}

pub fn get_global_staking_stats(ctx: Context<GetGlobalStats>) -> Result<()> {
    let stats = &ctx.accounts.global_staking_stats;
    
    require!(stats.initialized, SnakeError::Unauthorized);
    
    msg!("=== Global Staking Statistics ===");
    msg!("Total Users: {}", stats.total_users);
    msg!("Total Stakers: {}", stats.total_stakers);
    msg!("Total Patrons: {}", stats.total_patrons);
    msg!("Total Locked Amount: {}", stats.total_locked_amount);
    msg!("Total Yield Distributed: {}", stats.total_yield_distributed);
    msg!("Last Updated: {}", stats.last_updated);
    
    Ok(())
}