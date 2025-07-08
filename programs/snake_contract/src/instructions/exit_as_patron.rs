use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Burn};
use crate::{
    state::{UserClaim, UserRole, PatronStatus},
    events::PatronExited,
    errors::SnakeError,
    constants::PATRON_EXIT_BURN_PERCENT,
};

#[derive(Accounts)]
pub struct ExitAsPatron<'info> {
    #[account(mut)]
    pub patron: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", patron.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
        constraint = user_claim.role == UserRole::Patron @ SnakeError::OnlyApprovedPatrons,
        constraint = user_claim.patron_status == PatronStatus::Approved @ SnakeError::PatronNotApproved,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        constraint = patron_token_account.owner == patron.key(),
    )]
    pub patron_token_account: Account<'info, TokenAccount>,
    
    /// Treasury token account for buyback
    #[account(
        mut,
        seeds = [b"treasury"],
        bump,
    )]
    pub treasury_pda: SystemAccount<'info>,
    
    #[account(
        mut,
        constraint = treasury_token_account.owner == treasury_pda.key(),
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub mint: Account<'info, anchor_spl::token::Mint>,
    
    pub token_program: Program<'info, Token>,
}

pub fn exit_as_patron(ctx: Context<ExitAsPatron>, amount: u64) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if patron can exit (not during lock period for Month 6 commitment)
    let six_months_in_seconds = 6 * 30 * 24 * 60 * 60; // Approximate
    let _commitment_end = user_claim.patron_approval_timestamp + six_months_in_seconds;
    
    // Allow exit before Month 6 but with penalties
    require!(amount > 0, SnakeError::InsufficientFunds);
    require!(
        ctx.accounts.patron_token_account.amount >= amount,
        SnakeError::InsufficientFunds
    );
    
    // Calculate burn amount (20%) and remaining amount (80%)
    let burn_amount = amount
        .checked_mul(PATRON_EXIT_BURN_PERCENT)
        .ok_or(SnakeError::ArithmeticOverflow)?
        .checked_div(100)
        .ok_or(SnakeError::ArithmeticOverflow)?;
    
    let remaining_amount = amount
        .checked_sub(burn_amount)
        .ok_or(SnakeError::ArithmeticOverflow)?;
    
    // Burn 20% of tokens
    let cpi_burn = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.patron_token_account.to_account_info(),
        authority: ctx.accounts.patron.to_account_info(),
    };
    
    let burn_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_burn,
    );
    
    token::burn(burn_ctx, burn_amount)?;
    
    // Transfer remaining 80% to treasury for DAO buyback
    let cpi_transfer = Transfer {
        from: ctx.accounts.patron_token_account.to_account_info(),
        to: ctx.accounts.treasury_token_account.to_account_info(),
        authority: ctx.accounts.patron.to_account_info(),
    };
    
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_transfer,
    );
    
    token::transfer(transfer_ctx, remaining_amount)?;
    
    // Update patron status - they become normal user but marked as early seller
    user_claim.role = UserRole::None;
    user_claim.patron_status = PatronStatus::Revoked;
    user_claim.sold_early = true;
    
    emit!(PatronExited {
        user: ctx.accounts.patron.key(),
        burn_amount,
        remaining_amount,
        timestamp: current_time,
    });
    
    Ok(())
}