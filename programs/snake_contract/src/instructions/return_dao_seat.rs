use anchor_lang::prelude::*;
use crate::{
    state::{UserClaim, DaoRegistry},
    events::DAOSeatReturned,
    errors::SnakeError,
};

#[derive(Accounts)]
pub struct ReturnDAOSeat<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
        constraint = user_claim.dao_seat_holder @ SnakeError::NotDAOSeatHolder,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
}

pub fn return_dao_seat(ctx: Context<ReturnDAOSeat>) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let dao_registry = &mut ctx.accounts.dao_registry;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Return seat to treasury
    dao_registry.release_seat();
    
    // Update user claim
    user_claim.dao_seat_holder = false;
    user_claim.dao_seat_acquired_timestamp = 0;
    
    emit!(DAOSeatReturned {
        user: ctx.accounts.user.key(),
        timestamp: current_time,
    });
    
    Ok(())
}