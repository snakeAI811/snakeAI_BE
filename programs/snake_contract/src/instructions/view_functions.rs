use anchor_lang::prelude::*;
use crate::{
    state::{UserClaim, DaoRegistry, UserRole, PatronStatus},
};

#[derive(Accounts)]
pub struct CheckDAOEligibility<'info> {
    /// CHECK: This is the user whose DAO eligibility is being checked
    pub user: UncheckedAccount<'info>,
    
    #[account(
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
}

pub fn check_dao_eligibility_view(ctx: Context<CheckDAOEligibility>) -> Result<bool> {
    let user_claim = &ctx.accounts.user_claim;
    let dao_registry = &ctx.accounts.dao_registry;
    let current_time = Clock::get()?.unix_timestamp;
    
    let is_eligible = crate::instructions::acquire_dao_seat::check_dao_eligibility(
        user_claim,
        dao_registry,
        current_time,
    )?;
    
    Ok(is_eligible)
}

#[derive(Accounts)]
pub struct IsPatron<'info> {
    /// CHECK: This is the user whose patron status is being checked
    pub user: UncheckedAccount<'info>,
    
    #[account(
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
    )]
    pub user_claim: Account<'info, UserClaim>,
}

pub fn is_patron(ctx: Context<IsPatron>) -> Result<bool> {
    let user_claim = &ctx.accounts.user_claim;
    Ok(user_claim.role == UserRole::Patron && user_claim.patron_status == PatronStatus::Approved)
}

#[derive(Accounts)]
pub struct GetDAOMembers<'info> {
    #[account(
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
}

pub fn get_dao_info(ctx: Context<GetDAOMembers>) -> Result<(u32, u32, u64)> {
    let dao_registry = &ctx.accounts.dao_registry;
    Ok((
        dao_registry.total_seats,
        dao_registry.occupied_seats,
        dao_registry.min_stake_for_eligibility,
    ))
}

#[derive(Accounts)]
pub struct GetUserInfo<'info> {
    /// CHECK: This is the user whose info is being retrieved
    pub user: UncheckedAccount<'info>,
    
    #[account(
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
    )]
    pub user_claim: Account<'info, UserClaim>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UserInfo {
    pub role: UserRole,
    pub patron_status: PatronStatus,
    pub locked_amount: u64,
    pub lock_end_timestamp: i64,
    pub dao_eligible: bool,
    pub dao_seat_holder: bool,
    pub sold_early: bool,
    pub total_yield_claimed: u64,
}

pub fn get_user_info(ctx: Context<GetUserInfo>) -> Result<UserInfo> {
    let user_claim = &ctx.accounts.user_claim;
    
    Ok(UserInfo {
        role: user_claim.role.clone(),
        patron_status: user_claim.patron_status.clone(),
        locked_amount: user_claim.locked_amount,
        lock_end_timestamp: user_claim.lock_end_timestamp,
        dao_eligible: user_claim.dao_eligible,
        dao_seat_holder: user_claim.dao_seat_holder,
        sold_early: user_claim.sold_early,
        total_yield_claimed: user_claim.total_yield_claimed,
    })
}