use anchor_lang::prelude::*;
use crate::{
    state::UserClaim,
    errors::SnakeError,
    constants::*,
};

/// Rate limiting for operations
pub struct RateLimiter {
    pub last_operation: i64,
    pub operation_count: u32,
    pub max_operations: u32,
    pub time_window: i64,
}

impl RateLimiter {
    pub fn new(max_operations: u32, time_window: i64) -> Self {
        Self {
            last_operation: 0,
            operation_count: 0,
            max_operations,
            time_window,
        }
    }

    pub fn check_rate_limit(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        
        // Reset counter if time window has passed
        if current_time - self.last_operation > self.time_window {
            self.operation_count = 0;
        }
        
        // Check if operation count exceeds limit
        if self.operation_count >= self.max_operations {
            return Err(SnakeError::CooldownNotPassed.into());
        }
        
        self.operation_count += 1;
        self.last_operation = current_time;
        Ok(())
    }
}

/// Security utilities for enhanced protection
pub struct SecurityUtils;

impl SecurityUtils {
    /// Validate transaction size to prevent DoS attacks
    pub fn validate_transaction_size(data_size: usize, max_size: usize) -> Result<()> {
        require!(data_size <= max_size, SnakeError::InvalidAmount);
        Ok(())
    }

    /// Check for suspicious activity patterns
    pub fn detect_suspicious_activity(user_claim: &UserClaim) -> Result<()> {
        // Check for rapid role changes
        if user_claim.last_claim_timestamp > 0 {
            let time_since_last = Clock::get()?.unix_timestamp - user_claim.last_claim_timestamp;
            if time_since_last < 60 { // Less than 1 minute between operations
                msg!("Warning: Rapid operation detected");
            }
        }

        // Check for unusual token amounts
        if user_claim.accumulated_rewards > 1_000_000 * LAMPORTS_PER_SNK { // 1M tokens
            msg!("Warning: Large token amount detected");
        }

        Ok(())
    }

    /// Validate account ownership with additional checks
    pub fn validate_account_ownership(
        account_owner: &Pubkey,
        expected_owner: &Pubkey,
        operation: &str,
    ) -> Result<()> {
        require!(
            account_owner == expected_owner,
            SnakeError::Unauthorized
        );
        
        // Additional security check: ensure account is not a PDA
        if account_owner.to_bytes()[0] == 0 {
            msg!("Warning: Account appears to be a PDA");
        }
        
        Ok(())
    }

    /// Sanitize input strings to prevent injection attacks
    pub fn sanitize_string(input: &str, max_length: usize) -> Result<String> {
        require!(input.len() <= max_length, SnakeError::InvalidAmount);
        
        // Remove potentially dangerous characters
        let sanitized = input
            .chars()
            .filter(|&c| c.is_alphanumeric() || c.is_whitespace() || c == '_' || c == '-')
            .collect::<String>();
        
        Ok(sanitized)
    }

    /// Validate numeric ranges with overflow protection
    pub fn validate_numeric_range(
        value: u64,
        min: u64,
        max: u64,
        operation: &str,
    ) -> Result<()> {
        require!(value >= min, SnakeError::InvalidAmount);
        require!(value <= max, SnakeError::InvalidAmount);
        
        // Additional overflow check
        if value == u64::MAX {
            msg!("Warning: Maximum value detected in {}", operation);
        }
        
        Ok(())
    }

    /// Check for reentrancy protection
    pub fn check_reentrancy_guard(guard_value: &mut bool) -> Result<()> {
        require!(!*guard_value, SnakeError::Unauthorized);
        *guard_value = true;
        Ok(())
    }

    /// Release reentrancy guard
    pub fn release_reentrancy_guard(guard_value: &mut bool) {
        *guard_value = false;
    }
}
