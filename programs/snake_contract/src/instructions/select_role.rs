use anchor_lang::prelude::*;
use crate::state::{UserClaim, UserRole, PatronStatus};
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
    
    // Ensure user has mined tokens in Phase 1
    require!(user_claim.total_mined_phase1 > 0, SnakeError::NoMiningHistory);
    
    // Validate role transition based on new 3-path system
    match (&user_claim.role, &role) {
        (UserRole::None, UserRole::None) => {
            // User stays as default (Seller path) - can sell at TGE
        },
        (UserRole::None, UserRole::Staker) => {
            // Seller -> Staker: Lock tokens for 3 months, earn 5% APY
            user_claim.lock_duration_months = 3;
            let current_time = Clock::get()?.unix_timestamp;
            user_claim.lock_start_timestamp = current_time;
            user_claim.lock_end_timestamp = current_time + (3 * 30 * 24 * 60 * 60);
        },
        (UserRole::None, UserRole::Patron) => {
            // Seller -> Patron: Must have applied and been approved in Phase 1
            require!(
                user_claim.patron_status == PatronStatus::Approved,
                SnakeError::PatronNotApproved
            );
            
            // Set 6-month commitment period
            user_claim.lock_duration_months = 6;
            let current_time = Clock::get()?.unix_timestamp;
            user_claim.lock_start_timestamp = current_time;
            user_claim.lock_end_timestamp = current_time + (6 * 30 * 24 * 60 * 60);
        },
        (UserRole::Staker, UserRole::Patron) => {
            // Staker -> Patron: Must have applied and been approved
            require!(
                user_claim.patron_status == PatronStatus::Approved,
                SnakeError::PatronNotApproved
            );
            
            // Upgrade to 6-month commitment
            user_claim.lock_duration_months = 6;
            let current_time = Clock::get()?.unix_timestamp;
            user_claim.lock_end_timestamp = current_time + (6 * 30 * 24 * 60 * 60);
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
