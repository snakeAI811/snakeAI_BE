use anchor_lang::prelude::*;
use crate::state::{UserClaim, UserRole, PatronStatus};
use crate::errors::SnakeError;
use crate::constants::{
    PATRON_MIN_WALLET_AGE_DAYS,
    STAKER_MIN_STAKING_MONTHS,
    PATRON_MIN_STAKING_MONTHS
};

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

/// Helper function to check if user has sufficient staking history
fn has_sufficient_staking_history(user_claim: &UserClaim, required_months: u8) -> bool {
    if user_claim.lock_start_timestamp == 0 {
        return false; // No staking history
    }
    
    let current_time = Clock::get().unwrap().unix_timestamp;
    let staking_duration_seconds = current_time.saturating_sub(user_claim.lock_start_timestamp);
    let required_seconds = (required_months as i64) * 30 * 24 * 60 * 60; // Approximate months to seconds
    
    staking_duration_seconds >= required_seconds
}

/// Helper function to validate patron eligibility for role selection
fn validate_patron_eligibility_for_role(user_claim: &UserClaim) -> bool {
    // Check minimum mining threshold (> 0)
    if user_claim.total_mined_phase1 == 0 {
        return false;
    }
    
    // Check wallet age (>= 30 days)
    if user_claim.wallet_age_days < PATRON_MIN_WALLET_AGE_DAYS {
        return false;
    }
    
    // For role selection, we don't require the full 6 months staking history yet
    // That will be checked during lock_tokens instead
    // But user should have some staking experience if transitioning from Staker role
    true
}

pub fn select_role(ctx: Context<SelectRole>, role: UserRole) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    require!(user_claim.initialized, SnakeError::Unauthorized);
    
    // Note: Role selection validates eligibility criteria but doesn't lock tokens
    // Token locking happens separately in the lock_tokens instruction
    
    // Validate role transition based on new 3-path system
    match (&user_claim.role, &role) {
        (UserRole::None, UserRole::None) => {
            // User stays as default (Seller path) - can sell at TGE
        },
        (UserRole::None, UserRole::Staker) => {
            // Seller -> Staker: No special requirements for role selection
            // Actual staking validation will happen in lock_tokens
        },
        (UserRole::None, UserRole::Patron) => {
            // Seller -> Patron: Must meet patron eligibility criteria
            require!(
                validate_patron_eligibility_for_role(user_claim),
                SnakeError::PatronEligibilityNotMet
            );
            
            require!(
                user_claim.patron_status == PatronStatus::Approved,
                SnakeError::PatronNotApproved
            );
        },
        (UserRole::Staker, UserRole::Patron) => {
            // Staker -> Patron: Must have staking history and meet patron criteria
            require!(
                has_sufficient_staking_history(user_claim, PATRON_MIN_STAKING_MONTHS),
                SnakeError::InsufficientStakingHistory
            );
            
            require!(
                validate_patron_eligibility_for_role(user_claim),
                SnakeError::PatronEligibilityNotMet
            );
            
            require!(
                user_claim.patron_status == PatronStatus::Approved,
                SnakeError::PatronNotApproved
            );
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
