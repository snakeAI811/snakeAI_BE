use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    state::{VestingAccount, VestingRoleType, RewardPool},
    events::{VestingCreated, VestingWithdrawn},
    errors::SnakeError,
};

#[derive(Accounts)]
#[instruction(amount: u64, role_type: VestingRoleType)]
pub struct CreateVesting<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        init,
        payer = user,
        space = 8 + VestingAccount::INIT_SPACE,
        seeds = [b"vesting", user.key().as_ref()],
        bump,
    )]
    pub vesting_account: Account<'info, VestingAccount>,
    
    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.amount >= amount @ SnakeError::InsufficientFunds,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// Escrow token account to hold locked tokens
    #[account(
        mut,
        seeds = [b"vesting_escrow", user.key().as_ref()],
        bump,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn create_vesting(
    ctx: Context<CreateVesting>,
    amount: u64,
    role_type: VestingRoleType,
) -> Result<()> {
    require!(amount > 0, SnakeError::CannotLockZeroTokens);
    
    let vesting_account = &mut ctx.accounts.vesting_account;
    vesting_account.init(
        ctx.accounts.user.key(),
        amount,
        role_type.clone(),
        ctx.bumps.vesting_account,
    )?;
    
    // Transfer tokens to escrow
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.escrow_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    
    token::transfer(cpi_ctx, amount)?;
    
    emit!(VestingCreated {
        user: ctx.accounts.user.key(),
        amount,
        role_type,
        start_timestamp: vesting_account.start_timestamp,
        unlock_timestamp: vesting_account.unlock_timestamp,
    });
    
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawVesting<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"vesting", user.key().as_ref()],
        bump = vesting_account.bump,
        constraint = vesting_account.user == user.key() @ SnakeError::Unauthorized,
        constraint = vesting_account.can_withdraw() @ SnakeError::VestingNotUnlocked,
    )]
    pub vesting_account: Account<'info, VestingAccount>,
    
    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// Escrow token account holding locked tokens
    #[account(
        mut,
        seeds = [b"vesting_escrow", user.key().as_ref()],
        bump,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    
    /// Reward pool for yield distribution (stakers only)
    #[account(
        mut,
        seeds = [b"reward_pool"],
        bump,
    )]
    pub reward_pool: Account<'info, RewardPool>,
    
    /// Treasury token account for yield distribution
    #[account(
        mut,
        constraint = treasury_token_account.owner == reward_pool.treasury,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn withdraw_vesting(ctx: Context<WithdrawVesting>) -> Result<()> {
    let vesting_account = &mut ctx.accounts.vesting_account;
    let user_key = ctx.accounts.user.key();
    
    // Calculate yield for stakers
    let yield_amount = vesting_account.calculate_yield()?;
    let total_withdrawal = vesting_account.amount + yield_amount;
    
    // Create PDA signer seeds
    let signer_seeds = &[
        b"vesting_escrow",
        user_key.as_ref(),
        &[ctx.bumps.escrow_token_account],
    ];
    let signer = &[&signer_seeds[..]];
    
    // Transfer principal from escrow to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.escrow_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.escrow_token_account.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer,
    );
    
    token::transfer(cpi_ctx, vesting_account.amount)?;
    
    // Transfer yield from treasury token account (stakers only)
    if yield_amount > 0 {
        // For now, we'll skip the yield transfer since we need proper treasury setup
        // This would require a treasury PDA setup which is beyond the current scope
        msg!("Yield calculation: {} tokens (transfer skipped - requires treasury PDA setup)", yield_amount);
    }
    
    vesting_account.withdrawn = true;
    
    emit!(VestingWithdrawn {
        user: user_key,
        amount: vesting_account.amount,
        yield_amount,
        total_withdrawal,
    });
    
    Ok(())
}

#[derive(Accounts)]
pub struct AdminForceExit<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// CHECK: Target user account
    pub target_user: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"vesting", target_user.key().as_ref()],
        bump = vesting_account.bump,
        constraint = vesting_account.user == target_user.key() @ SnakeError::Unauthorized,
    )]
    pub vesting_account: Account<'info, VestingAccount>,
    
    #[account(
        mut,
        constraint = target_token_account.owner == target_user.key(),
    )]
    pub target_token_account: Account<'info, TokenAccount>,
    
    /// Escrow token account holding locked tokens
    #[account(
        mut,
        seeds = [b"vesting_escrow", target_user.key().as_ref()],
        bump,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn admin_force_exit(ctx: Context<AdminForceExit>) -> Result<()> {
    let vesting_account = &mut ctx.accounts.vesting_account;
    let target_user_key = ctx.accounts.target_user.key();
    
    // Only allow admin to force exit
    // TODO: Add proper admin validation
    
    // Mark early exit for patrons
    vesting_account.mark_early_exit();
    
    // Create PDA signer seeds
    let signer_seeds = &[
        b"vesting_escrow",
        target_user_key.as_ref(),
        &[ctx.bumps.escrow_token_account],
    ];
    let signer = &[&signer_seeds[..]];
    
    // Transfer tokens back to user (forced exit)
    let cpi_accounts = Transfer {
        from: ctx.accounts.escrow_token_account.to_account_info(),
        to: ctx.accounts.target_token_account.to_account_info(),
        authority: ctx.accounts.escrow_token_account.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer,
    );
    
    token::transfer(cpi_ctx, vesting_account.amount)?;
    
    vesting_account.withdrawn = true;
    
    emit!(VestingWithdrawn {
        user: target_user_key,
        amount: vesting_account.amount,
        yield_amount: 0, // No yield for forced exit
        total_withdrawal: vesting_account.amount,
    });
    
    Ok(())
}