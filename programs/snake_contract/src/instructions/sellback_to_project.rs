use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount, Transfer};
use crate::state::{UserClaim, UserRole, PatronStatus};


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
    let user_claim = &mut ctx.accounts.user_claim;
    
    // Check if user is a Patron - if so, apply special rules
    if user_claim.role == UserRole::Patron {
        // Patron exit logic: 20% burn + remove privileges
        let burn_amount = amount.checked_mul(20).unwrap().checked_div(100).unwrap(); // 20%
        let transfer_amount = amount.checked_sub(burn_amount).unwrap(); // 80%

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

        // Mark as early seller and revoke Patron privileges
        user_claim.sold_early = true;
        user_claim.role = UserRole::None; // Revert to normal user
        user_claim.patron_status = PatronStatus::Revoked;
        
        // Remove DAO seat if they had one
        if user_claim.dao_seat_holder {
            user_claim.dao_seat_holder = false;
            user_claim.dao_eligible = false;
        }
    } else {
        // Regular user sellback - no burn penalty
        let cpi_transfer = Transfer {
            from: ctx.accounts.patron_token_account.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(),
            authority: ctx.accounts.patron.to_account_info(),
        };
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_transfer), amount)?;
        
        // Mark as early seller
        user_claim.sold_early = true;
    }

    Ok(())
}
