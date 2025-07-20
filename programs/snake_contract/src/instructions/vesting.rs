use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::{UserClaim, UserRole, PatronStatus};
use crate::events::{VestingScheduleCreated, TokensVested};
use crate::errors::SnakeError;

#[derive(Accounts)]
pub struct CreateVesting<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.user == user.key()
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        constraint = user_token_account.owner == user.key()
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = user,
        space = VestingSchedule::INIT_SPACE,
        seeds = [b"vesting", user.key().as_ref()],
        bump
    )]
    pub vesting_schedule: Account<'info, VestingSchedule>,
    
    /// CHECK: This is the vesting escrow account that will hold the tokens
    #[account(
        mut,
        constraint = vesting_escrow.owner == vesting_schedule.key()
    )]
    pub vesting_escrow: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawVesting<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.user == user.key()
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"vesting", user.key().as_ref()],
        bump,
        constraint = vesting_schedule.beneficiary == user.key()
    )]
    pub vesting_schedule: Account<'info, VestingSchedule>,
    
    #[account(
        mut,
        constraint = vesting_escrow.owner == vesting_schedule.key()
    )]
    pub vesting_escrow: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = user_token_account.owner == user.key()
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct VestingSchedule {
    pub beneficiary: Pubkey,
    pub total_amount: u64,
    pub vested_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub duration_months: u8,
    pub vesting_type: VestingType,
    pub is_active: bool,
    pub last_claim_time: i64,
    pub yield_rate: u16, // APY in basis points (500 = 5%)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum VestingType {
    Staker,   // 3 months with 5% APY
    Patron,   // 6 months commitment
}

impl Default for VestingType {
    fn default() -> Self {
        Self::Staker
    }
}

pub fn create_vesting_schedule(
    ctx: Context<CreateVesting>,
    vesting_amount: u64,
) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let vesting_schedule = &mut ctx.accounts.vesting_schedule;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Determine vesting parameters based on user role
    let (duration_months, yield_rate, vesting_type) = match user_claim.role {
        UserRole::Staker => (3u8, 500u16, VestingType::Staker), // 3 months, 5% APY
        UserRole::Patron => (6u8, 0u16, VestingType::Patron),   // 6 months, no yield
        UserRole::None => return Err(SnakeError::InvalidRole.into()),
    };
    
    // Calculate end time
    let seconds_per_month = 30 * 24 * 60 * 60;
    let end_time = current_time + (duration_months as i64 * seconds_per_month);
    
    // For Patrons, enforce their commitment rules
    if vesting_type == VestingType::Patron {
        require!(
            user_claim.patron_status == PatronStatus::Approved,
            SnakeError::NotApprovedPatron
        );
        
        require!(
            !user_claim.sold_early,
            SnakeError::PatronSoldEarly
        );
    }
    
    // Initialize vesting schedule
    vesting_schedule.beneficiary = ctx.accounts.user.key();
    vesting_schedule.total_amount = vesting_amount;
    vesting_schedule.vested_amount = 0;
    vesting_schedule.start_time = current_time;
    vesting_schedule.end_time = end_time;
    vesting_schedule.cliff_time = current_time; // No cliff for simplicity
    vesting_schedule.duration_months = duration_months;
    vesting_schedule.vesting_type = vesting_type.clone();
    vesting_schedule.is_active = true;
    vesting_schedule.last_claim_time = current_time;
    vesting_schedule.yield_rate = yield_rate;
    
    // Transfer tokens to vesting escrow
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vesting_escrow.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    
    token::transfer(transfer_ctx, vesting_amount)?;
    
    // Update user claim lock details
    user_claim.locked_amount = vesting_amount;
    user_claim.lock_start_timestamp = current_time;
    user_claim.lock_end_timestamp = end_time;
    user_claim.lock_duration_months = duration_months;
    
    emit!(VestingScheduleCreated {
        beneficiary: ctx.accounts.user.key(),
        total_amount: vesting_amount,
        duration_months,
        vesting_type,
        start_time: current_time,
        end_time,
    });
    
    Ok(())
}

pub fn claim_vested_tokens(ctx: Context<WithdrawVesting>) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let vesting_schedule = &mut ctx.accounts.vesting_schedule;
    let current_time = Clock::get()?.unix_timestamp;
    
    require!(
        vesting_schedule.is_active,
        SnakeError::VestingNotActive
    );
    
    // Calculate vested amount
    let vested_amount = if current_time >= vesting_schedule.end_time {
        // Fully vested
        vesting_schedule.total_amount
    } else if current_time <= vesting_schedule.cliff_time {
        // Before cliff
        0
    } else {
        // Linear vesting
        let elapsed = current_time - vesting_schedule.start_time;
        let duration = vesting_schedule.end_time - vesting_schedule.start_time;
        
        (vesting_schedule.total_amount as u128)
            .checked_mul(elapsed as u128)
            .ok_or(SnakeError::ArithmeticOverflow)?
            .checked_div(duration as u128)
            .ok_or(SnakeError::ArithmeticOverflow)? as u64
    };
    
    // Calculate yield for stakers
    let yield_amount = if vesting_schedule.vesting_type == VestingType::Staker {
        let time_since_last_claim = current_time - vesting_schedule.last_claim_time;
        let seconds_in_year = 365 * 24 * 60 * 60;
        
        (vesting_schedule.total_amount as u128)
            .checked_mul(vesting_schedule.yield_rate as u128)
            .ok_or(SnakeError::ArithmeticOverflow)?
            .checked_mul(time_since_last_claim as u128)
            .ok_or(SnakeError::ArithmeticOverflow)?
            .checked_div(10000) // basis points
            .ok_or(SnakeError::ArithmeticOverflow)?
            .checked_div(seconds_in_year as u128)
            .ok_or(SnakeError::ArithmeticOverflow)? as u64
    } else {
        0
    };
    
    let claimable_amount = vested_amount.saturating_sub(vesting_schedule.vested_amount);
    let total_claim = claimable_amount.saturating_add(yield_amount);
    
    require!(
        total_claim > 0,
        SnakeError::NothingToClaim
    );
    
    // Transfer tokens from escrow to user
    let seeds = &[
        b"vesting",
        vesting_schedule.beneficiary.as_ref(),
        &[ctx.bumps.vesting_schedule],
    ];
    let signer = &[&seeds[..]];
    
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vesting_escrow.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: vesting_schedule.to_account_info(),
        },
        signer,
    );
    
    token::transfer(transfer_ctx, total_claim)?;
    
    // Update vesting schedule
    vesting_schedule.vested_amount = vested_amount;
    vesting_schedule.last_claim_time = current_time;
    
    // Update user claim yield tracking
    if vesting_schedule.vesting_type == VestingType::Staker {
        user_claim.last_yield_claim_timestamp = current_time;
        user_claim.total_yield_claimed = user_claim.total_yield_claimed.saturating_add(yield_amount);
    }
    
    // Mark as complete if fully vested
    if vested_amount >= vesting_schedule.total_amount {
        vesting_schedule.is_active = false;
        
        // Update user claim lock status
        user_claim.locked_amount = 0;
        
        // For Patrons, check if they can be DAO eligible after 6 months
        if vesting_schedule.vesting_type == VestingType::Patron && 
           !user_claim.sold_early && 
           user_claim.mined_in_phase2 {
            user_claim.dao_eligible = true;
        }
    }
    
    emit!(TokensVested {
        beneficiary: vesting_schedule.beneficiary,
        amount: claimable_amount,
        yield_amount,
        total_vested: vesting_schedule.vested_amount,
    });
    
    Ok(())
}
