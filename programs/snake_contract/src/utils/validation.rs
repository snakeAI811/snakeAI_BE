use anchor_lang::prelude::*;
use crate::{
    state::{UserClaim, UserRole, PatronStatus},
    errors::SnakeError,
    constants::*,
};

/// Common validation functions used across multiple instructions
pub struct ValidationUtils;

impl ValidationUtils {
    /// Validate user has sufficient tokens for operation
    pub fn validate_sufficient_tokens(user_claim: &UserClaim, required_amount: u64) -> Result<()> {
        require!(
            user_claim.claimable_amount >= required_amount,
            SnakeError::InsufficientFunds
        );
        Ok(())
    }

    /// Validate user role for specific operations
    pub fn validate_user_role(user_claim: &UserClaim, allowed_roles: &[UserRole]) -> Result<()> {
        require!(
            allowed_roles.contains(&user_claim.role),
            SnakeError::Unauthorized
        );
        Ok(())
    }

    /// Validate patron status for patron-specific operations
    pub fn validate_patron_status(user_claim: &UserClaim) -> Result<()> {
        require!(
            user_claim.role == UserRole::Patron,
            SnakeError::OnlyPatronsCanSell
        );
        require!(
            user_claim.patron_status == PatronStatus::Approved,
            SnakeError::OnlyPatronsCanSell
        );
        Ok(())
    }

    /// Validate lock status for staking operations
    pub fn validate_lock_status(user_claim: &UserClaim, should_be_locked: bool) -> Result<()> {
        let is_locked = user_claim.is_locked();
        if should_be_locked {
            require!(is_locked, SnakeError::TokensNotLocked);
        } else {
            require!(!is_locked, SnakeError::TokensLocked);
        }
        Ok(())
    }

    /// Validate cooldown period for operations like yield claiming
    pub fn validate_cooldown(
        last_operation_timestamp: i64,
        cooldown_seconds: i64,
    ) -> Result<()> {
        if last_operation_timestamp > 0 {
            let current_time = Clock::get()?.unix_timestamp;
            let time_since_last = current_time - last_operation_timestamp;
            require!(
                time_since_last >= cooldown_seconds,
                SnakeError::YieldClaimCooldownNotPassed
            );
        }
        Ok(())
    }

    /// Validate amount is within acceptable range
    pub fn validate_amount_range(amount: u64, min: u64, max: u64) -> Result<()> {
        require!(amount >= min, SnakeError::InvalidAmount);
        require!(amount <= max, SnakeError::InvalidAmount);
        Ok(())
    }

    /// Validate patron qualification criteria
    pub fn validate_patron_qualification(user_claim: &UserClaim) -> Result<()> {
        require!(
            user_claim.total_mined_phase1 >= 1_000_000_000_000, // 1000 tokens
            SnakeError::InsufficientMiningHistory
        );
        require!(
            user_claim.wallet_age_days >= PATRON_MIN_WALLET_AGE_DAYS,
            SnakeError::InsufficientWalletAge
        );
        require!(
            user_claim.community_score >= 50, // Minimum community score
            SnakeError::InsufficientCommunityScore
        );
        Ok(())
    }
}
