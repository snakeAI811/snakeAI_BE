use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Burn, Mint};
use crate::{
    state::{UserClaim, UserRole, OtcSwap, RewardPool},
    errors::SnakeError,
    utils::CalculationUtils,
    constants::*,
};

/// Deflationary mechanics for OTC swaps
pub struct DeflationaryMechanics;

impl DeflationaryMechanics {
    /// Calculate burn amount based on swap type and user role
    pub fn calculate_burn_amount(
        token_amount: u64,
        swap_type: &crate::state::SwapType,
        user_claim: &UserClaim,
        current_time: i64,
    ) -> u64 {
        match swap_type {
            crate::state::SwapType::ExiterToPatron => {
                // Phase 1: No burn for Exiter → Patron swaps
                0
            },
            crate::state::SwapType::ExiterToTreasury => {
                // Phase 1: Small burn for treasury fallback (1%)
                (token_amount * 100) / BASIS_POINTS
            },
            crate::state::SwapType::PatronToPatron => {
                // Phase 2: 20% burn for patron exits
                if user_claim.role == UserRole::Patron {
                    CalculationUtils::calculate_patron_exit_burn(token_amount)
                } else {
                    0
                }
            }
        }
    }

    /// Calculate treasury skim amount
    pub fn calculate_treasury_skim(
        token_amount: u64,
        swap_type: &crate::state::SwapType,
    ) -> u64 {
        match swap_type {
            crate::state::SwapType::ExiterToPatron => {
                // Phase 1: 2% treasury skim
                (token_amount * 200) / BASIS_POINTS
            },
            crate::state::SwapType::ExiterToTreasury => {
                // Phase 1: 5% treasury skim for fallback
                (token_amount * 500) / BASIS_POINTS
            },
            crate::state::SwapType::PatronToPatron => {
                // Phase 2: No treasury skim for P2P
                0
            }
        }
    }

    /// Execute burn and treasury skim operations
    pub fn execute_deflationary_operations<'info>(
        token_amount: u64,
        burn_amount: u64,
        treasury_skim: u64,
        seller_token_account: &Account<'info, TokenAccount>,
        treasury_account: &Account<'info, TokenAccount>,
        mint: &Account<'info, Mint>,
        authority: &AccountInfo<'info>,
        token_program: &Program<'info, Token>,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        // Execute burn if applicable
        if burn_amount > 0 {
            Self::execute_token_burn(
                seller_token_account,
                mint,
                authority,
                burn_amount,
                token_program,
                signer_seeds,
            )?;
        }

        // Execute treasury skim if applicable
        if treasury_skim > 0 {
            Self::execute_treasury_skim(
                seller_token_account,
                treasury_account,
                authority,
                treasury_skim,
                token_program,
                signer_seeds,
            )?;
        }

        Ok(())
    }

    /// Execute token burn
    fn execute_token_burn<'info>(
        token_account: &Account<'info, TokenAccount>,
        mint: &Account<'info, Mint>,
        authority: &AccountInfo<'info>,
        amount: u64,
        token_program: &Program<'info, Token>,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let cpi_accounts = Burn {
            mint: mint.to_account_info(),
            from: token_account.to_account_info(),
            authority: authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        token::burn(cpi_ctx, amount)?;
        Ok(())
    }

    /// Execute treasury skim transfer
    fn execute_treasury_skim<'info>(
        from_account: &Account<'info, TokenAccount>,
        to_account: &Account<'info, TokenAccount>,
        authority: &AccountInfo<'info>,
        amount: u64,
        token_program: &Program<'info, Token>,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let cpi_accounts = Transfer {
            from: from_account.to_account_info(),
            to: to_account.to_account_info(),
            authority: authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

/// Daily volume tracking and controls
#[account]
#[derive(Default, InitSpace)]
pub struct DailyVolumeTracker {
    pub date: i64,                    // Date in YYYYMMDD format
    pub total_volume: u64,            // Total volume for the day
    pub max_daily_volume: u64,        // Maximum allowed daily volume
    pub swap_count: u64,              // Number of swaps today
    pub last_reset: i64,              // Last reset timestamp
    pub bump: u8,
}

impl DailyVolumeTracker {
    pub fn init(&mut self, max_daily_volume: u64, bump: u8) {
        self.date = Self::get_current_date();
        self.total_volume = 0;
        self.max_daily_volume = max_daily_volume;
        self.swap_count = 0;
        self.last_reset = Clock::get().unwrap().unix_timestamp;
        self.bump = bump;
    }

    /// Get current date in YYYYMMDD format
    fn get_current_date() -> i64 {
        let timestamp = Clock::get().unwrap().unix_timestamp;
        // Convert timestamp to days since epoch, then to YYYYMMDD format
        let days_since_epoch = timestamp / (24 * 60 * 60);
        // Simple conversion: 1970-01-01 + days
        let year = 1970 + (days_since_epoch / 365);
        let day_of_year = days_since_epoch % 365;
        let month = 1 + (day_of_year / 30); // Approximate
        let day = 1 + (day_of_year % 30);
        
        year * 10000 + month * 100 + day
    }

    /// Check and reset daily volume if needed
    pub fn check_and_reset_daily_volume(&mut self) -> Result<()> {
        let current_date = Self::get_current_date();
        
        if current_date != self.date {
            // New day, reset counters
            self.date = current_date;
            self.total_volume = 0;
            self.swap_count = 0;
            self.last_reset = Clock::get()?.unix_timestamp;
            msg!("Daily volume reset for date: {}", current_date);
        }
        
        Ok(())
    }

    /// Add volume and check limits
    pub fn add_volume(&mut self, amount: u64) -> Result<()> {
        self.check_and_reset_daily_volume()?;
        
        // Check if adding this amount would exceed daily limit
        let new_total = self.total_volume.checked_add(amount)
            .ok_or(SnakeError::ArithmeticOverflow)?;
        
        require!(
            new_total <= self.max_daily_volume,
            SnakeError::MaxOTCLimitExceeded
        );
        
        self.total_volume = new_total;
        self.swap_count = self.swap_count.checked_add(1)
            .ok_or(SnakeError::ArithmeticOverflow)?;
        
        Ok(())
    }

    /// Get current daily volume status
    pub fn get_volume_status(&self) -> (u64, u64, u64) {
        (self.total_volume, self.max_daily_volume, self.swap_count)
    }
}
