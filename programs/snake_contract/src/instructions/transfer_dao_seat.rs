use anchor_lang::prelude::*;
use crate::{
    state::{UserClaim, DaoRegistry},
    events::DAOSeatTransferred,
    errors::SnakeError,
};

#[derive(Accounts)]
pub struct TransferDAOSeat<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    
    /// CHECK: This is the recipient of the DAO seat
    pub to: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", from.key().as_ref()],
        bump,
        constraint = from_claim.initialized @ SnakeError::Unauthorized,
        constraint = from_claim.dao_seat_holder @ SnakeError::NotDAOSeatHolder,
    )]
    pub from_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"user_claim", to.key().as_ref()],
        bump,
        constraint = to_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub to_claim: Account<'info, UserClaim>,
    
    #[account(
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
}

pub fn transfer_dao_seat(ctx: Context<TransferDAOSeat>) -> Result<()> {
    let from_claim = &mut ctx.accounts.from_claim;
    let to_claim = &mut ctx.accounts.to_claim;
    let dao_registry = &ctx.accounts.dao_registry;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if recipient is eligible for DAO seat
    let is_eligible = crate::instructions::acquire_dao_seat::check_dao_eligibility(
        to_claim, 
        dao_registry, 
        current_time
    )?;
    require!(is_eligible, SnakeError::NotEligibleForDAO);
    
    // Check if recipient already has a seat
    require!(!to_claim.dao_seat_holder, SnakeError::NotEligibleForDAO);
    
    // Transfer seat
    from_claim.dao_seat_holder = false;
    from_claim.dao_seat_acquired_timestamp = 0;
    
    to_claim.dao_seat_holder = true;
    to_claim.dao_eligible = true;
    to_claim.dao_seat_acquired_timestamp = current_time;
    
    emit!(DAOSeatTransferred {
        from: ctx.accounts.from.key(),
        to: ctx.accounts.to.key(),
        timestamp: current_time,
    });
    
    Ok(())
}