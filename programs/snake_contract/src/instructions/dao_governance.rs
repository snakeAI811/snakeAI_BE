use anchor_lang::prelude::*;

use crate::state::{UserClaim, UserRole};
use crate::events::{DAOSeatAllocated, DAOSeatRevoked};
use crate::errors::SnakeError;

#[derive(Accounts)]
pub struct AllocateDAOSeat<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.user == user.key()
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    /// CHECK: The user who will receive the DAO seat
    pub user: AccountInfo<'info>,
    
    #[account(
        init_if_needed,
        payer = authority,
        space = DAOSeat::INIT_SPACE,
        seeds = [b"dao_seat", user.key().as_ref()],
        bump
    )]
    pub dao_seat: Account<'info, DAOSeat>,
    
    #[account(
        mut,
        seeds = [b"dao_registry"],
        bump
    )]
    pub dao_registry: Account<'info, DAORegistry>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokeDAOSeat<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.user == user.key()
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    /// CHECK: The user whose DAO seat will be revoked
    pub user: AccountInfo<'info>,
    
    #[account(
        mut,
        seeds = [b"dao_seat", user.key().as_ref()],
        bump,
        constraint = dao_seat.holder == user.key()
    )]
    pub dao_seat: Account<'info, DAOSeat>,
    
    #[account(
        mut,
        seeds = [b"dao_registry"],
        bump
    )]
    pub dao_registry: Account<'info, DAORegistry>,
}

#[derive(Accounts)]
pub struct CheckDAOEligibility<'info> {
    #[account(
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.user == user.key()
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    /// CHECK: The user to check eligibility for
    pub user: AccountInfo<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct DAOSeat {
    pub holder: Pubkey,
    pub allocated_at: i64,
    pub is_active: bool,
    pub voting_power: u64,
    pub role: UserRole,
    pub patron_score: u32,
}

#[account]
#[derive(InitSpace)]
pub struct DAORegistry {
    pub total_seats: u32,
    pub allocated_seats: u32,
    pub max_seats: u32,
    pub min_dao_stake: u64,
    pub month6_timestamp: i64, // When Month 6 begins for DAO eligibility
}

impl Default for DAORegistry {
    fn default() -> Self {
        Self {
            total_seats: 0,
            allocated_seats: 0,
            max_seats: 100, // Configurable maximum
            min_dao_stake: 1000 * 1_000_000_000, // 1000 SNAKE tokens
            month6_timestamp: 0,
        }
    }
}

pub fn allocate_dao_seat(
    ctx: Context<AllocateDAOSeat>,
    current_balance: u64,
) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let dao_seat = &mut ctx.accounts.dao_seat;
    let dao_registry = &mut ctx.accounts.dao_registry;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if Month 6 has passed
    require!(
        current_time >= dao_registry.month6_timestamp,
        SnakeError::Month6NotReached
    );
    
    // Check if user meets DAO eligibility criteria
    let is_eligible = user_claim.check_month6_dao_eligibility(current_balance, dao_registry.min_dao_stake);
    
    require!(
        is_eligible,
        SnakeError::NotEligibleForDAO
    );
    
    require!(
        dao_registry.allocated_seats < dao_registry.max_seats,
        SnakeError::MaxSeatsReached
    );
    
    // Calculate voting power based on role and holdings
    let voting_power = match user_claim.role {
        UserRole::Patron => {
            // Patrons get higher voting power
            current_balance + (user_claim.patron_qualification_score as u64 * 1_000_000_000)
        },
        UserRole::Staker => {
            // Stakers get voting power based on stake
            current_balance
        },
        UserRole::None => return Err(SnakeError::InvalidRole.into()),
    };
    
    // Initialize DAO seat
    dao_seat.holder = ctx.accounts.user.key();
    dao_seat.allocated_at = current_time;
    dao_seat.is_active = true;
    dao_seat.voting_power = voting_power;
    dao_seat.role = user_claim.role.clone();
    dao_seat.patron_score = user_claim.patron_qualification_score;
    
    // Update user claim
    user_claim.dao_seat_holder = true;
    user_claim.dao_seat_acquired_timestamp = current_time;
    
    // Update registry
    dao_registry.allocated_seats += 1;
    
    emit!(DAOSeatAllocated {
        holder: ctx.accounts.user.key(),
        allocated_at: current_time,
        voting_power,
        role: user_claim.role.clone(),
    });
    
    Ok(())
}

pub fn revoke_dao_seat(ctx: Context<RevokeDAOSeat>, reason: String) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let dao_seat = &mut ctx.accounts.dao_seat;
    let dao_registry = &mut ctx.accounts.dao_registry;
    
    require!(
        dao_seat.is_active,
        SnakeError::SeatNotActive
    );
    
    // Deactivate seat
    dao_seat.is_active = false;
    
    // Update user claim
    user_claim.dao_seat_holder = false;
    user_claim.dao_eligible = false;
    
    // Update registry
    dao_registry.allocated_seats = dao_registry.allocated_seats.saturating_sub(1);
    
    emit!(DAOSeatRevoked {
        holder: ctx.accounts.user.key(),
        revoked_at: Clock::get()?.unix_timestamp,
        reason,
    });
    
    Ok(())
}

pub fn check_dao_eligibility(ctx: Context<CheckDAOEligibility>, current_balance: u64) -> Result<bool> {
    let user_claim = &ctx.accounts.user_claim;
    
    // This is a view function to check eligibility without state changes
    let is_eligible = user_claim.check_month6_dao_eligibility(current_balance, 1000 * 1_000_000_000);
    
    Ok(is_eligible)
}

// Helper function to initialize DAO registry (called once)
#[derive(Accounts)]
pub struct InitializeDAORegistry<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = DAORegistry::INIT_SPACE,
        seeds = [b"dao_registry"],
        bump
    )]
    pub dao_registry: Account<'info, DAORegistry>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_dao_registry(
    ctx: Context<InitializeDAORegistry>,
    max_seats: u32,
    min_dao_stake: u64,
    month6_timestamp: i64,
) -> Result<()> {
    let dao_registry = &mut ctx.accounts.dao_registry;
    
    dao_registry.total_seats = 0;
    dao_registry.allocated_seats = 0;
    dao_registry.max_seats = max_seats;
    dao_registry.min_dao_stake = min_dao_stake;
    dao_registry.month6_timestamp = month6_timestamp;
    
    Ok(())
}

// Helper to get all DAO seats (for frontend)
// Helper function commented out for now - needs proper program type
// pub fn get_all_dao_seats(program: &Program<crate::SnakeContract>) -> Result<Vec<DAOSeat>> {
//     let dao_seats = program.account::<DAOSeat>()
//         .all()
//         .map_err(|_| SnakeError::FailedToFetchSeats)?
//         .into_iter()
//         .filter(|seat| seat.is_active)
//         .collect();
//     
//     Ok(dao_seats)
// }
