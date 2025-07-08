use anchor_lang::prelude::*;
use crate::{
    state::{UserClaim, DaoRegistry, UserRole, PatronStatus},
    events::{DAOEligibilityAcquired, DAOSeatAcquired},
    errors::SnakeError,
};

#[derive(Accounts)]
pub struct AcquireDAOSeat<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
}

pub fn acquire_dao_seat(ctx: Context<AcquireDAOSeat>) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let dao_registry = &mut ctx.accounts.dao_registry;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if user is already a DAO seat holder
    require!(!user_claim.dao_seat_holder, SnakeError::NotEligibleForDAO);
    
    // Check DAO eligibility
    let is_eligible = check_dao_eligibility(user_claim, dao_registry, current_time)?;
    require!(is_eligible, SnakeError::NotEligibleForDAO);
    
    // Check if there are available seats
    require!(dao_registry.has_available_seats(), SnakeError::NoAvailableSeats);
    
    // Acquire seat
    dao_registry.acquire_seat()?;
    
    // Update user claim
    user_claim.dao_eligible = true;
    user_claim.dao_seat_holder = true;
    user_claim.dao_seat_acquired_timestamp = current_time;
    
    emit!(DAOEligibilityAcquired {
        user: ctx.accounts.user.key(),
        timestamp: current_time,
    });
    
    emit!(DAOSeatAcquired {
        user: ctx.accounts.user.key(),
        seat_number: dao_registry.occupied_seats,
        timestamp: current_time,
    });
    
    Ok(())
}

pub fn check_dao_eligibility(
    user_claim: &UserClaim,
    dao_registry: &DaoRegistry,
    current_time: i64,
) -> Result<bool> {
    // Must be either Patron or Staker
    if user_claim.role != UserRole::Patron && user_claim.role != UserRole::Staker {
        return Ok(false);
    }
    
    // For Patrons: must be approved and completed 6-month commitment
    if user_claim.role == UserRole::Patron {
        if user_claim.patron_status != PatronStatus::Approved {
            return Ok(false);
        }
        
        // Check if 6-month commitment is completed
        let six_months_seconds = 6 * 30 * 24 * 60 * 60;
        let commitment_end = user_claim.patron_approval_timestamp + six_months_seconds;
        
        if current_time < commitment_end {
            return Ok(false);
        }
        
        // Must not have sold early
        if user_claim.sold_early {
            return Ok(false);
        }
    }
    
    // For Stakers: must have completed lock period and have minimum stake
    if user_claim.role == UserRole::Staker {
        // Must have locked tokens for required duration
        if user_claim.lock_duration_months < dao_registry.lock_duration_requirement_months {
            return Ok(false);
        }
        
        // Must have completed the lock period
        if current_time < user_claim.lock_end_timestamp {
            return Ok(false);
        }
        
        // Must have minimum stake amount (check locked amount as proxy)
        if user_claim.locked_amount < dao_registry.min_stake_for_eligibility {
            return Ok(false);
        }
    }
    
    Ok(true)
}