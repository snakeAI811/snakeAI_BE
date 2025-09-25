use anchor_lang::prelude::*;
use crate::{
    state::{UserClaim, PatronStatus, RewardPool},
    events::PatronApproved,
    errors::SnakeError,
};

#[derive(Accounts)]
pub struct ConfirmPatron<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        seeds = [b"reward_pool"],
        bump,
        constraint = reward_pool.admin == admin.key() @ SnakeError::Unauthorized,
    )]
    pub reward_pool: Account<'info, RewardPool>,
    
    /// CHECK: This is the user whose patron status is being confirmed
    pub user: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub user_claim: Account<'info, UserClaim>,
}

pub fn confirm_patron(ctx: Context<ConfirmPatron>) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if user has applied as patron
    require!(
        user_claim.patron_status == PatronStatus::Applied,
        SnakeError::PatronNotApproved
    );
    
    // Update patron status to Approved
    user_claim.patron_status = PatronStatus::Approved;
    user_claim.patron_approval_timestamp = current_time;
    
    emit!(PatronApproved {
        user: ctx.accounts.user.key(),
        approved_by: ctx.accounts.admin.key(),
        timestamp: current_time,
    });
    
    Ok(())
}