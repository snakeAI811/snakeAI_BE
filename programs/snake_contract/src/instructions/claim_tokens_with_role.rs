use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
use crate::state::{UserClaim, UserRole};

#[derive(Accounts)]
pub struct ClaimTokensWithRole<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(mut)]
    pub user_token_ata: Account<'info, TokenAccount>,
    
    /// Treasury PDA that has authority over the treasury token account
    #[account(
        seeds = [b"treasury"],
        bump,
    )]
    pub treasury_pda: SystemAccount<'info>,
    
    /// Treasury token account owned by the treasury PDA
    #[account(
        mut,
        constraint = treasury_token_account.owner == treasury_pda.key(),
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    pub token_program: Program<'info, Token>,
}

pub fn claim_tokens_with_role(ctx: Context<ClaimTokensWithRole>, amount: u64, role: UserRole) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    
    require!(user_claim.initialized, CustomError::UserNotInitialized);
    require!(user_claim.role == UserRole::None, CustomError::RoleAlreadySelected);
    
    // Set the user's role
    user_claim.role = role;
    
    // Create signer seeds for treasury PDA
    let treasury_bump = ctx.bumps.treasury_pda;
    let treasury_signer_seeds: &[&[u8]] = &[
        b"treasury",
        &[treasury_bump],
    ];
    let treasury_signer = &[treasury_signer_seeds];
    
    // Transfer tokens from treasury to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.treasury_token_account.to_account_info(),
        to: ctx.accounts.user_token_ata.to_account_info(),
        authority: ctx.accounts.treasury_pda.to_account_info(), // Treasury PDA signs
    };
    
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), 
        cpi_accounts,
        treasury_signer, // Provide PDA signer
    );
    
    token::transfer(cpi_ctx, amount)?;
    
    Ok(())
}

#[error_code]
pub enum CustomError {
    #[msg("User not initialized")] 
    UserNotInitialized,
    #[msg("Role already selected")] 
    RoleAlreadySelected,
}