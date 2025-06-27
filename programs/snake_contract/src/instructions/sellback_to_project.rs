use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount, Transfer};
use crate::state::{UserClaim, UserRole};

#[derive(Accounts)]
pub struct SellbackToProject<'info> {
    #[account(mut)]
    pub patron: Signer<'info>,
    #[account(mut)]
    pub patron_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, anchor_spl::token::Mint>,
    #[account(mut, seeds = [b"user_claim", patron.key().as_ref()], bump)]
    pub user_claim: Account<'info, UserClaim>,
    pub token_program: Program<'info, Token>,
}

pub fn sellback_to_project(ctx: Context<SellbackToProject>, amount: u64) -> Result<()> {
    let burn_amount = amount / 5; // 20%
    let transfer_amount = amount - burn_amount; // 80%

    // Burn 20%
    let cpi_burn = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.patron_token_account.to_account_info(),
        authority: ctx.accounts.patron.to_account_info(),
    };
    token::burn(CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_burn), burn_amount)?;

    // Transfer 80% to treasury
    let cpi_transfer = Transfer {
        from: ctx.accounts.patron_token_account.to_account_info(),
        to: ctx.accounts.treasury_token_account.to_account_info(),
        authority: ctx.accounts.patron.to_account_info(),
    };
    token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_transfer), transfer_amount)?;

    // Mark Patron as exited (update state)
    ctx.accounts.user_claim.role = UserRole::Exit;

    Ok(())
}
