use anchor_lang::prelude::*;
use crate::{
    state::{DaoRegistry, RewardPool},
    events::DAORegistryInitialized,
    errors::SnakeError,
    constants::{DAO_TOTAL_SEATS, MIN_DAO_STAKE_AMOUNT},
};

#[derive(Accounts)]
pub struct InitializeDAORegistry<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        seeds = [b"reward_pool"],
        bump,
        constraint = reward_pool.admin == admin.key() @ SnakeError::Unauthorized,
    )]
    pub reward_pool: Account<'info, RewardPool>,
    
    #[account(
        init,
        payer = admin,
        space = 8 + DaoRegistry::INIT_SPACE,
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_dao_registry(
    ctx: Context<InitializeDAORegistry>,
    total_seats: Option<u32>,
    min_stake: Option<u64>,
) -> Result<()> {
    let dao_registry = &mut ctx.accounts.dao_registry;
    
    let seats = total_seats.unwrap_or(DAO_TOTAL_SEATS);
    let min_stake_amount = min_stake.unwrap_or(MIN_DAO_STAKE_AMOUNT);
    
    dao_registry.init(seats, min_stake_amount);
    
    emit!(DAORegistryInitialized {
        total_seats: seats,
        min_stake: min_stake_amount,
    });
    
    Ok(())
}