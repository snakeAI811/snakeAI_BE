use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Burn};

use crate::state::{UserClaim, UserRole, PatronStatus};
use crate::events::PatronExited;
use crate::errors::SnakeError;

#[derive(Accounts)]
pub struct PatronExit<'info> {
    #[account(mut)]
    pub patron: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", patron.key().as_ref()],
        bump,
        constraint = patron_claim.user == patron.key(),
        constraint = patron_claim.role == UserRole::Patron,
        constraint = patron_claim.patron_status == PatronStatus::Approved
    )]
    pub patron_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        constraint = patron_token_account.owner == patron.key()
    )]
    pub patron_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: This is the token mint account, verified by constraint
    #[account(
        mut,
        constraint = token_mint.key() == patron_token_account.mint
    )]
    pub token_mint: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct PatronOTCExit<'info> {
    #[account(mut)]
    pub patron: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", patron.key().as_ref()],
        bump,
        constraint = patron_claim.user == patron.key(),
        constraint = patron_claim.role == UserRole::Patron,
        constraint = patron_claim.patron_status == PatronStatus::Approved
    )]
    pub patron_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        constraint = patron_token_account.owner == patron.key()
    )]
    pub patron_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: This is the treasury or other patron's token account
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: This is the payment account for the patron
    #[account(mut)]
    pub patron_payment_account: Account<'info, TokenAccount>,
    
    /// CHECK: This is the buyer's payment account
    #[account(mut)]
    pub buyer_payment_account: Account<'info, TokenAccount>,
    
    /// CHECK: This is the token mint account, verified by constraint
    #[account(
        mut,
        constraint = token_mint.key() == patron_token_account.mint
    )]
    pub token_mint: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
}

pub fn patron_exit(ctx: Context<PatronExit>, exit_amount: u64) -> Result<()> {
    let patron_claim = &mut ctx.accounts.patron_claim;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if patron is still within commitment period (6 months)
    let six_months_seconds = 6 * 30 * 24 * 60 * 60;
    let commitment_end = patron_claim.patron_approval_timestamp + six_months_seconds;
    
    if current_time < commitment_end {
        // Early exit - apply 20% burn penalty
        let burn_amount = exit_amount
            .checked_mul(20)
            .ok_or(SnakeError::ArithmeticOverflow)?
            .checked_div(100)
            .ok_or(SnakeError::ArithmeticOverflow)?;
        
        // Burn 20% of the exit amount
        let burn_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.token_mint.to_account_info(),
                from: ctx.accounts.patron_token_account.to_account_info(),
                authority: ctx.accounts.patron.to_account_info(),
            },
        );
        
        token::burn(burn_ctx, burn_amount)?;
        
        // Mark patron as having sold early
        patron_claim.sold_early = true;
        patron_claim.dao_eligible = false;
        patron_claim.dao_seat_holder = false;
        
        emit!(PatronExited {
            patron: ctx.accounts.patron.key(),
            exit_amount,
            burn_amount,
            early_exit: true,
        });
        
        msg!("Patron exited early. Burned {} tokens (20% penalty)", burn_amount);
    } else {
        // Normal exit after commitment period - no penalty
        emit!(PatronExited {
            patron: ctx.accounts.patron.key(),
            exit_amount,
            burn_amount: 0,
            early_exit: false,
        });
        
        msg!("Patron completed commitment period. No burn penalty applied.");
    }
    
    Ok(())
}

pub fn patron_otc_exit(
    ctx: Context<PatronOTCExit>,
    exit_amount: u64,
    sale_price: u64,
) -> Result<()> {
    let patron_claim = &mut ctx.accounts.patron_claim;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if patron is still within commitment period (6 months)
    let six_months_seconds = 6 * 30 * 24 * 60 * 60;
    let commitment_end = patron_claim.patron_approval_timestamp + six_months_seconds;
    
    if current_time < commitment_end {
        // Early exit - apply 20% burn penalty
        let burn_amount = exit_amount
            .checked_mul(20)
            .ok_or(SnakeError::ArithmeticOverflow)?
            .checked_div(100)
            .ok_or(SnakeError::ArithmeticOverflow)?;
        
        let transferable_amount = exit_amount
            .checked_sub(burn_amount)
            .ok_or(SnakeError::ArithmeticOverflow)?;
        
        // Burn 20% of the exit amount
        let burn_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.token_mint.to_account_info(),
                from: ctx.accounts.patron_token_account.to_account_info(),
                authority: ctx.accounts.patron.to_account_info(),
            },
        );
        
        token::burn(burn_ctx, burn_amount)?;
        
        // Transfer remaining 80% to buyer
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.patron_token_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.patron.to_account_info(),
            },
        );
        
        token::transfer(transfer_ctx, transferable_amount)?;
        
        // Transfer payment to patron
        let payment_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer_payment_account.to_account_info(),
                to: ctx.accounts.patron_payment_account.to_account_info(),
                authority: ctx.accounts.patron.to_account_info(),
            },
        );
        
        token::transfer(payment_ctx, sale_price)?;
        
        // Mark patron as having sold early and revoke DAO privileges
        patron_claim.sold_early = true;
        patron_claim.dao_eligible = false;
        patron_claim.dao_seat_holder = false;
        
        emit!(PatronExited {
            patron: ctx.accounts.patron.key(),
            exit_amount,
            burn_amount,
            early_exit: true,
        });
        
        msg!("Patron OTC exit: {} tokens transferred, {} tokens burned", transferable_amount, burn_amount);
    } else {
        // Normal exit after commitment period - no burn penalty
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.patron_token_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.patron.to_account_info(),
            },
        );
        
        token::transfer(transfer_ctx, exit_amount)?;
        
        // Transfer payment to patron
        let payment_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer_payment_account.to_account_info(),
                to: ctx.accounts.patron_payment_account.to_account_info(),
                authority: ctx.accounts.patron.to_account_info(),
            },
        );
        
        token::transfer(payment_ctx, sale_price)?;
        
        emit!(PatronExited {
            patron: ctx.accounts.patron.key(),
            exit_amount,
            burn_amount: 0,
            early_exit: false,
        });
        
        msg!("Patron completed commitment. Full {} tokens transferred", exit_amount);
    }
    
    Ok(())
}
