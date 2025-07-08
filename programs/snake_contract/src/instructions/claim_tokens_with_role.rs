use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
use crate::{
    errors::SnakeError,
    state::{UserClaim, UserRole, RewardPool},
};

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
    
    /// Reward Pool PDA that has authority over the treasury token account
    #[account(
        seeds = [b"reward_pool"],
        bump,
    )]
    pub reward_pool_pda: Account<'info, RewardPool>,
    
    /// Treasury token account owned by the reward pool PDA
    #[account(
        mut,
        constraint = treasury_token_account.owner == reward_pool_pda.key() @ SnakeError::InvalidTreasuryAuthority,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    pub token_program: Program<'info, Token>,
}

pub fn claim_tokens_with_role(ctx: Context<ClaimTokensWithRole>, amount: u64, role: UserRole) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    
    require!(user_claim.initialized, SnakeError::Unauthorized);
    require!(user_claim.role == UserRole::None, SnakeError::InvalidRoleTransition);
    
    // Set the user's role
    user_claim.role = role;
    
    // Create signer seeds for reward pool PDA
    let reward_pool_bump = ctx.bumps.reward_pool_pda;
    let reward_pool_signer_seeds: &[&[u8]] = &[
        b"reward_pool",
        &[reward_pool_bump],
    ];
    let reward_pool_signer = &[reward_pool_signer_seeds];
    
    // Transfer tokens from treasury to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.treasury_token_account.to_account_info(),
        to: ctx.accounts.user_token_ata.to_account_info(),
        authority: ctx.accounts.reward_pool_pda.to_account_info(), // Reward Pool PDA signs
    };
    
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), 
        cpi_accounts,
        reward_pool_signer, // Provide PDA signer
    );
    
    token::transfer(cpi_ctx, amount)?;
    
    Ok(())
}

