use anchor_lang::prelude::*;
use crate::state::{UserClaim, UserRole};
use crate::errors::SnakeError;

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
    require!(user_claim.initialized, SnakeError::Unauthorized);
    
    // Validate role transition
    match (&user_claim.role, &role) {
        (UserRole::None, _) => {
            // Normal users can select any role
        },
        (UserRole::Staker, UserRole::Patron) => {
            // Stakers can upgrade to Patron
        },
        (current_role, new_role) if current_role == new_role => {
            // Same role is allowed (no change)
        },
        _ => {
            return Err(SnakeError::InvalidRoleTransition.into());
        }
    }
    
    user_claim.role = role;
    Ok(())
}
