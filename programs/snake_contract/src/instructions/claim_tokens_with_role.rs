use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer, Burn};
use crate::{
    errors::SnakeError,
    state::{UserClaim, UserRole, RewardPool, ClaimReceipt},
};
use anchor_lang::solana_program::hash::hash;

fn hash_tweet_id(tweet_id: &str) -> [u8; 32] {
    hash(tweet_id.as_bytes()).to_bytes()
}

#[derive(Accounts)]
#[instruction(amount: u64, role: UserRole, tweet_id: String)]
pub struct ClaimTokensWithRole<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        has_one = user @ SnakeError::Unauthorized,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub user_claim: Account<'info, UserClaim>,

    #[account(
        init,
        seeds = [b"claim_receipt", user.key().as_ref(), &hash_tweet_id(&tweet_id)],
        bump,
        payer = user,
        space = 8 + 32 + 64, // discriminator + claimer pubkey + tweet_id string
    )]
    pub claim_receipt: Account<'info, ClaimReceipt>,

    #[account(mut)]
    pub user_token_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"reward_pool"],
        bump
    )]
    pub reward_pool_pda: Account<'info, RewardPool>,

    #[account(
        mut,
        constraint = treasury_token_account.owner == reward_pool_pda.key() @ SnakeError::InvalidTreasuryAuthority,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn claim_tokens_with_role(ctx: Context<ClaimTokensWithRole>, amount: u64, role: UserRole, tweet_id: String) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;

    // Use the amount parameter as reward amount
    let reward_amount = amount;
    // For Twitter rewards, burn amount equals reward amount (1:1 ratio)
    let burn_amount = amount;

    // Transfer reward tokens from reward pool PDA to user
    let reward_pool_seeds = &[
        b"reward_pool".as_ref(),
        &[ctx.bumps.reward_pool_pda],
    ];
    let signer = &[&reward_pool_seeds[..]];

    let transfer_cpi_accounts = Transfer {
        from: ctx.accounts.treasury_token_account.to_account_info(),
        to: ctx.accounts.user_token_ata.to_account_info(),
        authority: ctx.accounts.reward_pool_pda.to_account_info(),
    };

    let transfer_cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_cpi_accounts,
        signer,
    );

    token::transfer(transfer_cpi_ctx, reward_amount)?;

    // Burn equivalent amount of tokens from treasury
    let burn_cpi_accounts = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.treasury_token_account.to_account_info(),
        authority: ctx.accounts.reward_pool_pda.to_account_info(),
    };

    let burn_cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        burn_cpi_accounts,
        signer,
    );

    token::burn(burn_cpi_ctx, burn_amount)?;

    // Save claim receipt for duplicate protection
    let receipt = &mut ctx.accounts.claim_receipt;
    receipt.claimer = ctx.accounts.user.key();
    receipt.tweet_id = tweet_id;

    // Set the user's role if provided
    user_claim.role = role;

    Ok(())
}
