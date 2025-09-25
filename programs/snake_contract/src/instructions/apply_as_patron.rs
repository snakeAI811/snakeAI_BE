use anchor_lang::prelude::*;
use crate::{
    state::{UserClaim, PatronStatus},
    events::PatronApplicationSubmitted,
    errors::SnakeError,
};

#[derive(Accounts)]
pub struct ApplyAsPatron<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub user_claim: Account<'info, UserClaim>,
}

pub fn apply_as_patron(ctx: Context<ApplyAsPatron>) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if user already has a patron application or is already approved
    require!(
        user_claim.patron_status == PatronStatus::None,
        SnakeError::PatronApplicationExists
    );
    
    // Calculate and check qualification score based on your requirements
    let qualification_score = user_claim.calculate_patron_qualification_score();
    
    // Minimum score requirement (can be adjusted)
    let min_score = 50u32; // Requires decent mining + wallet age + community contribution
    
    require!(
        qualification_score >= min_score,
        SnakeError::InsufficientQualificationScore
    );
    
    // Update patron status to Applied
    user_claim.patron_status = PatronStatus::Applied;
    user_claim.patron_application_timestamp = current_time;
    
    emit!(PatronApplicationSubmitted {
        user: ctx.accounts.user.key(),
        timestamp: current_time,
    });
    
    Ok(())
}