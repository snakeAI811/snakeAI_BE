use anchor_lang::prelude::*;
use crate::state::{UserClaim, UserRole};

#[derive(Accounts)]
pub struct SelectRole<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
    )]
    pub user_claim: Account<'info, UserClaim>,
}

pub fn select_role(ctx: Context<SelectRole>, role: UserRole) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    require!(user_claim.initialized, CustomError::UserNotInitialized);
    user_claim.role = role;
    Ok(())
}

#[error_code]
pub enum CustomError {
    #[msg("User not initialized")] 
    UserNotInitialized,
}
