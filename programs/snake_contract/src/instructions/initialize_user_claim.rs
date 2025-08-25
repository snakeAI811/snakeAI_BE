use anchor_lang::prelude::*;
use crate::state::UserClaim;

#[derive(Accounts)]
pub struct InitializeUserClaim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserClaim::INIT_SPACE,
        seeds = [b"user_claim", user.key().as_ref()],
        bump
    )]
    pub user_claim: Account<'info, UserClaim>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_user_claim(ctx: Context<InitializeUserClaim>) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    user_claim.init(ctx.accounts.user.key());
    Ok(())
}
