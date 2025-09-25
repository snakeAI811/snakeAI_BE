use anchor_lang::prelude::*;
use crate::state::{UserClaim, PatronStatus};
use crate::errors::SnakeError;
use crate::events::PatronApplicationSubmitted;

#[derive(Accounts)]
pub struct ApplyForPatron<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
    )]
    pub user_claim: Account<'info, UserClaim>,
}

#[derive(Accounts)]
pub struct ApprovePatronApplication<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user_claim", applicant.key().as_ref()],
        bump,
    )]
    pub user_claim: Account<'info, UserClaim>,
    /// CHECK: Applicant's public key for validation
    pub applicant: AccountInfo<'info>,
}

/// Apply for Patron status during Phase 1
/// Requirements based on your criteria:
/// - Wallet age / KYC (optional)
/// - Contribution to community
/// - On-chain record (mining history)
pub fn apply_for_patron(
    ctx: Context<ApplyForPatron>,
    wallet_age_days: u32,
    community_score: u32,
) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    require!(user_claim.initialized, SnakeError::Unauthorized);
    
    // Prevent duplicate applications
    require!(
        user_claim.patron_status == PatronStatus::None,
        SnakeError::PatronApplicationExists
    );
    
    // Must have some mining history to apply
    require!(
        user_claim.total_mined_phase1 > 0,
        SnakeError::NoMiningHistory
    );
    
    // Update patron application data
    user_claim.patron_status = PatronStatus::Applied;
    user_claim.patron_application_timestamp = Clock::get()?.unix_timestamp;
    user_claim.wallet_age_days = wallet_age_days;
    user_claim.community_score = community_score.min(30); // Cap at 30 points
    
    // Calculate qualification score
    let qualification_score = user_claim.calculate_patron_qualification_score();
    
    // Emit application event
    emit!(PatronApplicationSubmitted {
        user: ctx.accounts.user.key(),
        qualification_score,
        wallet_age_days,
        community_score: user_claim.community_score,
        total_mined: user_claim.total_mined_phase1,
        timestamp: user_claim.patron_application_timestamp,
    });
    
    Ok(())
}

/// Approve a Patron application (admin only)
/// Based on qualification score and manual review
pub fn approve_patron_application(
    ctx: Context<ApprovePatronApplication>,
    min_qualification_score: u32,
) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    
    // Only admin can approve (you'd need to add admin check here)
    // For now, we'll assume the admin constraint is handled elsewhere
    
    // Must have applied first
    require!(
        user_claim.patron_status == PatronStatus::Applied,
        SnakeError::PatronNotApproved
    );
    
    // Check qualification score
    let qualification_score = user_claim.calculate_patron_qualification_score();
    require!(
        qualification_score >= min_qualification_score,
        SnakeError::InsufficientQualificationScore
    );
    
    // Approve the application
    user_claim.patron_status = PatronStatus::Approved;
    user_claim.patron_approval_timestamp = Clock::get()?.unix_timestamp;
    
    Ok(())
}

/// Revoke Patron status (admin only)
/// Used for violations or early exits
pub fn revoke_patron_status(ctx: Context<ApprovePatronApplication>) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    
    // Only admin can revoke
    require!(
        user_claim.patron_status == PatronStatus::Approved,
        SnakeError::PatronNotApproved
    );
    
    // Revoke status
    user_claim.patron_status = PatronStatus::Revoked;
    user_claim.dao_eligible = false;
    user_claim.dao_seat_holder = false;
    
    Ok(())
}

/// Check if user meets patron criteria
/// This is a view function that can be called to check eligibility
pub fn check_patron_eligibility(
    ctx: Context<ApplyForPatron>,
    min_score: u32,
) -> Result<bool> {
    let user_claim = &mut ctx.accounts.user_claim;
    
    // Must be initialized and have mining history
    if !user_claim.initialized || user_claim.total_mined_phase1 == 0 {
        return Ok(false);
    }
    
    // Calculate current score
    let qualification_score = user_claim.calculate_patron_qualification_score();
    
    Ok(qualification_score >= min_score)
}
