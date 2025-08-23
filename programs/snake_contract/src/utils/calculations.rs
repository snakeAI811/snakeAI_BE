use anchor_lang::prelude::*;
use crate::{
    state::{UserClaim, UserRole},
    constants::*,
};

/// Common calculation functions used across multiple instructions
pub struct CalculationUtils;

impl CalculationUtils {
    /// Calculate yield for a user based on their role and locked amount
    pub fn calculate_yield(user_claim: &UserClaim, current_timestamp: i64) -> u64 {
        // Only Stakers and Patrons can earn yield
        if (user_claim.role != UserRole::Staker && user_claim.role != UserRole::Patron) 
            || user_claim.locked_amount == 0 {
            return 0;
        }

        let last_claim = if user_claim.last_yield_claim_timestamp == 0 {
            user_claim.lock_start_timestamp
        } else {
            user_claim.last_yield_claim_timestamp
        };

        let duration_months = user_claim.lock_duration_months;
        if duration_months <= 0 {
            return 0;
        }

        // APY based on role: Stakers get 5%, Patrons get 7%
        let apy_rate = match user_claim.role {
            UserRole::Staker => STAKER_APY as u64,
            UserRole::Patron => PATRON_APY as u64,
            _ => return 0,
        };

        // Calculate yield: (locked_amount * apy_rate * duration_months) / (100 * 12 months)
        // Use u128 to prevent overflow during calculation
        let calculation_result = (user_claim.locked_amount as u128)
            .saturating_mul(apy_rate as u128)
            .saturating_mul(duration_months as u128)
            .checked_div(100u128)
            .unwrap_or(0)
            .checked_div(12u128)
            .unwrap_or(0);

        if calculation_result > u64::MAX as u128 {
            u64::MAX
        } else {
            calculation_result as u64
        }
    }

    /// Calculate burn amount for patron exit penalty
    pub fn calculate_patron_exit_burn(amount: u64) -> u64 {
        (amount * PATRON_EXIT_BURN_PERCENT) / 100
    }

    /// Calculate rebate amount for OTC swaps
    pub fn calculate_rebate_amount(amount: u64, rebate_basis_points: u64) -> u64 {
        (amount * rebate_basis_points) / BASIS_POINTS
    }

    /// Calculate lock duration in seconds
    pub fn calculate_lock_duration_seconds(duration_months: u8) -> i64 {
        (duration_months as i64) * SECONDS_PER_MONTH
    }

    /// Calculate lock end timestamp
    pub fn calculate_lock_end_timestamp(start_timestamp: i64, duration_months: u8) -> i64 {
        start_timestamp + Self::calculate_lock_duration_seconds(duration_months)
    }

    /// Safe addition with overflow protection
    pub fn safe_add(a: u64, b: u64) -> Result<u64> {
        a.checked_add(b).ok_or_else(|| {
            msg!("Arithmetic overflow in addition");
            crate::errors::SnakeError::ArithmeticOverflow.into()
        })
    }

    /// Safe multiplication with overflow protection
    pub fn safe_mul(a: u64, b: u64) -> Result<u64> {
        a.checked_mul(b).ok_or_else(|| {
            msg!("Arithmetic overflow in multiplication");
            crate::errors::SnakeError::ArithmeticOverflow.into()
        })
    }

    /// Safe subtraction with underflow protection
    pub fn safe_sub(a: u64, b: u64) -> Result<u64> {
        a.checked_sub(b).ok_or_else(|| {
            msg!("Arithmetic underflow in subtraction");
            crate::errors::SnakeError::ArithmeticOverflow.into()
        })
    }

    /// Calculate patron qualification score
    pub fn calculate_patron_score(user_claim: &UserClaim) -> u32 {
        let mut score = 0u32;
        
        // Mining contribution (40 points max)
        if user_claim.total_mined_phase1 >= 1_000_000_000_000 { // 1000 tokens
            score += 40;
        } else if user_claim.total_mined_phase1 >= 500_000_000_000 { // 500 tokens
            score += 30;
        } else if user_claim.total_mined_phase1 >= 100_000_000_000 { // 100 tokens
            score += 20;
        } else if user_claim.total_mined_phase1 > 0 {
            score += 10;
        }
        
        // Wallet age (30 points max)
        if user_claim.wallet_age_days >= 90 { // 3+ months old
            score += 30;
        } else if user_claim.wallet_age_days >= 60 { // 2+ months old
            score += 20;
        } else if user_claim.wallet_age_days >= 30 { // 1+ month old
            score += 10;
        }
        
        // Community score (30 points max)
        score += user_claim.community_score.min(30);
        
        score
    }
}
