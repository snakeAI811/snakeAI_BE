use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Burn};
use crate::{
    state::{UserClaim, UserRole},
    events::{PatronExitTracked, TokensBurned, DAOEligibilityRevoked},
    errors::SnakeError,
};

/// Stub structure to track simulated OTC swap events
#[account]
#[derive(Default, InitSpace)]
pub struct OtcSwapTracker {
    pub user: Pubkey,
    pub total_swaps: u64,
    pub total_volume: u64,
    pub last_swap_timestamp: i64,
    pub exit_tracked: bool,
    pub burn_penalty_applied: bool,
    pub dao_eligibility_revoked: bool,
    pub bump: u8,
}

impl OtcSwapTracker {
    pub fn init(&mut self, user: Pubkey, bump: u8) {
        self.user = user;
        self.total_swaps = 0;
        self.total_volume = 0;
        self.last_swap_timestamp = 0;
        self.exit_tracked = false;
        self.burn_penalty_applied = false;
        self.dao_eligibility_revoked = false;
        self.bump = bump;
    }
    
    pub fn track_swap(&mut self, amount: u64) -> Result<()> {
        self.total_swaps += 1;
        self.total_volume += amount;
        self.last_swap_timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }
    
    pub fn mark_exit(&mut self) {
        self.exit_tracked = true;
    }
    
    pub fn apply_burn_penalty(&mut self) {
        self.burn_penalty_applied = true;
    }
    
    pub fn revoke_dao_eligibility(&mut self) {
        self.dao_eligibility_revoked = true;
    }
}

/// Simulate OTC swap event (Phase 2 stub)
#[derive(Accounts)]
pub struct SimulateOtcSwap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + OtcSwapTracker::INIT_SPACE,
        seeds = [b"otc_tracker", user.key().as_ref()],
        bump,
    )]
    pub otc_tracker: Account<'info, OtcSwapTracker>,
    
    pub system_program: Program<'info, System>,
}

pub fn simulate_otc_swap(
    ctx: Context<SimulateOtcSwap>,
    amount: u64,
    is_sale: bool,
) -> Result<()> {
    let user_claim = &ctx.accounts.user_claim;
    let otc_tracker = &mut ctx.accounts.otc_tracker;
    let user_key = ctx.accounts.user.key();
    
    // Initialize tracker if needed
    if !otc_tracker.exit_tracked && !otc_tracker.burn_penalty_applied {
        otc_tracker.init(user_key, ctx.bumps.otc_tracker);
    }
    
    // Track the swap
    otc_tracker.track_swap(amount)?;
    
    // If this is a sale and user is a patron, track exit
    if is_sale && user_claim.role == UserRole::Patron {
        otc_tracker.mark_exit();
        
        msg!("Patron sale detected - marking exit for user: {}", user_key);
        
        emit!(PatronExitTracked {
            user: user_key,
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });
    }
    
    Ok(())
}

/// Simulate burn on exit (Phase 2 stub - no real tokens burned)
#[derive(Accounts)]
pub struct SimulateBurnOnExit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
        constraint = user_claim.role == UserRole::Patron @ SnakeError::OnlyPatrons,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"otc_tracker", user.key().as_ref()],
        bump = otc_tracker.bump,
        constraint = otc_tracker.exit_tracked @ SnakeError::NoExitToTrack,
    )]
    pub otc_tracker: Account<'info, OtcSwapTracker>,
    
    pub system_program: Program<'info, System>,
}

pub fn simulate_burn_on_exit(
    ctx: Context<SimulateBurnOnExit>,
    exit_amount: u64,
) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let otc_tracker = &mut ctx.accounts.otc_tracker;
    let user_key = ctx.accounts.user.key();
    
    // Calculate 20% burn penalty
    let burn_amount = exit_amount
        .checked_mul(20)
        .ok_or(SnakeError::ArithmeticOverflow)?
        .checked_div(100)
        .ok_or(SnakeError::ArithmeticOverflow)?;
    
    // Mark burn penalty applied (simulation only)
    otc_tracker.apply_burn_penalty();
    
    // Revoke DAO eligibility
    user_claim.dao_eligible = false;
    user_claim.sold_early = true;
    otc_tracker.revoke_dao_eligibility();
    
    msg!("Simulated burn penalty: {} tokens (20% of {} exit amount)", burn_amount, exit_amount);
    msg!("DAO eligibility revoked for user: {}", user_key);
    
    emit!(TokensBurned {
        user: user_key,
        amount: burn_amount,
        reason: "patron_exit_penalty_simulated".to_string(),
    });
    
    emit!(DAOEligibilityRevoked {
        user: user_key,
        reason: "patron_early_exit".to_string(),
    });
    
    Ok(())
}

/// Real burn implementation (for Phase 3)
#[derive(Accounts)]
pub struct RealBurnOnExit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
        constraint = user_claim.role == UserRole::Patron @ SnakeError::OnlyPatrons,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"otc_tracker", user.key().as_ref()],
        bump = otc_tracker.bump,
        constraint = otc_tracker.exit_tracked @ SnakeError::NoExitToTrack,
    )]
    pub otc_tracker: Account<'info, OtcSwapTracker>,
    
    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    
    pub token_program: Program<'info, Token>,
}

pub fn real_burn_on_exit(
    ctx: Context<RealBurnOnExit>,
    exit_amount: u64,
) -> Result<()> {
    let user_claim = &mut ctx.accounts.user_claim;
    let otc_tracker = &mut ctx.accounts.otc_tracker;
    let user_key = ctx.accounts.user.key();
    
    // Calculate 20% burn penalty
    let burn_amount = exit_amount
        .checked_mul(20)
        .ok_or(SnakeError::ArithmeticOverflow)?
        .checked_div(100)
        .ok_or(SnakeError::ArithmeticOverflow)?;
    
    // Ensure user has sufficient tokens
    require!(
        ctx.accounts.user_token_account.amount >= burn_amount,
        SnakeError::InsufficientFunds
    );
    
    // Burn tokens
    let burn_accounts = Burn {
        mint: ctx.accounts.token_mint.to_account_info(),
        from: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    
    let burn_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        burn_accounts,
    );
    
    token::burn(burn_ctx, burn_amount)?;
    
    // Mark burn penalty applied
    otc_tracker.apply_burn_penalty();
    
    // Revoke DAO eligibility
    user_claim.dao_eligible = false;
    user_claim.sold_early = true;
    otc_tracker.revoke_dao_eligibility();
    
    emit!(TokensBurned {
        user: user_key,
        amount: burn_amount,
        reason: "patron_exit_penalty_real".to_string(),
    });
    
    emit!(DAOEligibilityRevoked {
        user: user_key,
        reason: "patron_early_exit".to_string(),
    });
    
    Ok(())
}

/// Get simulated swap statistics
#[derive(Accounts)]
pub struct GetSwapStats<'info> {
    /// CHECK: User account to check stats for
    pub user: UncheckedAccount<'info>,
    
    #[account(
        seeds = [b"otc_tracker", user.key().as_ref()],
        bump = otc_tracker.bump,
    )]
    pub otc_tracker: Account<'info, OtcSwapTracker>,
}

pub fn get_swap_stats(ctx: Context<GetSwapStats>) -> Result<(u64, u64, i64, bool, bool, bool)> {
    let tracker = &ctx.accounts.otc_tracker;
    
    Ok((
        tracker.total_swaps,
        tracker.total_volume,
        tracker.last_swap_timestamp,
        tracker.exit_tracked,
        tracker.burn_penalty_applied,
        tracker.dao_eligibility_revoked,
    ))
}

/// Mock UI event handler for testing
#[derive(Accounts)]
pub struct MockUIEvent<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
        constraint = user_claim.initialized @ SnakeError::Unauthorized,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"otc_tracker", user.key().as_ref()],
        bump = otc_tracker.bump,
    )]
    pub otc_tracker: Account<'info, OtcSwapTracker>,
}

pub fn mock_ui_event(
    ctx: Context<MockUIEvent>,
    event_type: String,
    amount: u64,
) -> Result<()> {
    let user_claim = &ctx.accounts.user_claim;
    let otc_tracker = &mut ctx.accounts.otc_tracker;
    let user_key = ctx.accounts.user.key();
    
    match event_type.as_str() {
        "patron_sale" => {
            if user_claim.role == UserRole::Patron {
                otc_tracker.mark_exit();
                otc_tracker.track_swap(amount)?;
                
                msg!("Mock UI Event: Patron sale detected - {} tokens", amount);
                
                emit!(PatronExitTracked {
                    user: user_key,
                    amount,
                    timestamp: Clock::get()?.unix_timestamp,
                });
            }
        },
        "normal_swap" => {
            otc_tracker.track_swap(amount)?;
            msg!("Mock UI Event: Normal swap - {} tokens", amount);
        },
        _ => {
            msg!("Mock UI Event: Unknown event type: {}", event_type);
        }
    }
    
    Ok(())
}