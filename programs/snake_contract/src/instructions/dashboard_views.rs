use anchor_lang::prelude::*;
use crate::{
    state::{UserClaim, DaoRegistry, UserRole, PatronStatus},
    errors::SnakeError,
};

/// Dashboard data structures for your social layer requirements

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UserPublicProfile {
    pub user: Pubkey,
    pub role: UserRole,
    pub patron_status: PatronStatus,
    pub dao_seat_holder: bool,
    pub total_mined_phase1: u64,
    pub mined_in_phase2: bool,
    pub locked_amount: u64,
    pub lock_end_timestamp: i64,
    pub qualification_score: u32,
    pub wallet_age_days: u32,
    pub community_score: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PatronInfo {
    pub user: Pubkey,
    pub approval_timestamp: i64,
    pub qualification_score: u32,
    pub dao_seat_holder: bool,
    pub commitment_end_timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SystemStats {
    pub total_dao_seats: u32,
    pub occupied_dao_seats: u32,
    pub min_dao_stake: u64,
}

// ========== VIEW FUNCTIONS ==========

#[derive(Accounts)]
pub struct GetUserProfile<'info> {
    /// CHECK: User whose profile is being viewed
    pub user: UncheckedAccount<'info>,
    
    #[account(
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub user_claim: Account<'info, UserClaim>,
}

#[derive(Accounts)]
pub struct GetSystemStats<'info> {
    #[account(
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
}

/// Get public profile of a user (for social layer display)
pub fn get_user_profile(ctx: Context<GetUserProfile>) -> Result<UserPublicProfile> {
    let user_claim = &ctx.accounts.user_claim;
    
    Ok(UserPublicProfile {
        user: ctx.accounts.user.key(),
        role: user_claim.role.clone(),
        patron_status: user_claim.patron_status.clone(),
        dao_seat_holder: user_claim.dao_seat_holder,
        total_mined_phase1: user_claim.total_mined_phase1,
        mined_in_phase2: user_claim.mined_in_phase2,
        locked_amount: user_claim.locked_amount,
        lock_end_timestamp: user_claim.lock_end_timestamp,
        qualification_score: user_claim.patron_qualification_score,
        wallet_age_days: user_claim.wallet_age_days,
        community_score: user_claim.community_score,
    })
}

/// Get patron-specific information
pub fn get_patron_info(ctx: Context<GetUserProfile>) -> Result<Option<PatronInfo>> {
    let user_claim = &ctx.accounts.user_claim;
    
    if user_claim.role == UserRole::Patron && user_claim.patron_status == PatronStatus::Approved {
        let six_months_seconds = 6 * 30 * 24 * 60 * 60;
        let commitment_end = user_claim.patron_approval_timestamp + six_months_seconds;
        
        Ok(Some(PatronInfo {
            user: ctx.accounts.user.key(),
            approval_timestamp: user_claim.patron_approval_timestamp,
            qualification_score: user_claim.patron_qualification_score,
            dao_seat_holder: user_claim.dao_seat_holder,
            commitment_end_timestamp: commitment_end,
        }))
    } else {
        Ok(None)
    }
}

/// Get system statistics for dashboard
pub fn get_system_stats(ctx: Context<GetSystemStats>) -> Result<SystemStats> {
    let dao_registry = &ctx.accounts.dao_registry;
    
    Ok(SystemStats {
        total_dao_seats: dao_registry.total_seats,
        occupied_dao_seats: dao_registry.occupied_seats,
        min_dao_stake: dao_registry.min_stake_for_eligibility,
    })
}

/// Check if user is a patron (public view)
pub fn is_user_patron(ctx: Context<GetUserProfile>) -> Result<bool> {
    let user_claim = &ctx.accounts.user_claim;
    Ok(user_claim.role == UserRole::Patron && user_claim.patron_status == PatronStatus::Approved)
}

/// Get user's current qualification score
pub fn get_qualification_score(ctx: Context<GetUserProfile>) -> Result<u32> {
    let user_claim = &ctx.accounts.user_claim;
    Ok(user_claim.patron_qualification_score)
}

/// Check Month 6 DAO eligibility status
pub fn check_dao_eligibility_status(ctx: Context<GetUserProfile>) -> Result<bool> {
    let user_claim = &ctx.accounts.user_claim;
    
    // This would need the user's current token balance
    // For now, using locked_amount as proxy
    let current_balance = user_claim.locked_amount;
    let min_dao_stake = 1_000_000_000; // From constants
    
    Ok(user_claim.check_month6_dao_eligibility(current_balance, min_dao_stake))
}