use anchor_lang::prelude::*;
use std::collections::HashMap;

/// Cache for frequently accessed data
pub struct Cache {
    pub timestamp: i64,
    pub calculations: HashMap<String, u64>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            timestamp: Clock::get().unwrap().unix_timestamp,
            calculations: HashMap::new(),
        }
    }

    /// Get cached calculation or compute and cache
    pub fn get_or_compute<F>(&mut self, key: &str, compute_fn: F) -> u64 
    where
        F: FnOnce() -> u64,
    {
        if let Some(&value) = self.calculations.get(key) {
            value
        } else {
            let value = compute_fn();
            self.calculations.insert(key.to_string(), value);
            value
        }
    }

    /// Update timestamp and clear stale cache
    pub fn update_timestamp(&mut self) {
        let new_timestamp = Clock::get().unwrap().unix_timestamp;
        if new_timestamp != self.timestamp {
            self.timestamp = new_timestamp;
            self.calculations.clear(); // Clear cache on timestamp change
        }
    }
}

/// Performance optimization utilities
pub struct PerformanceUtils;

impl PerformanceUtils {
    /// Batch token transfer operation
    pub fn batch_token_transfer(
        transfers: Vec<(Pubkey, u64)>,
        from_account: &AccountInfo,
        to_accounts: &[AccountInfo],
        authority: &AccountInfo,
        token_program: &AccountInfo,
    ) -> Result<()> {
        // This is a simplified version - in practice, you'd need to implement
        // the actual batch transfer logic based on your token program
        for (i, (amount, _)) in transfers.iter().enumerate() {
            if i < to_accounts.len() {
                // Individual transfer logic would go here
                msg!("Batch transfer: {} tokens to account {}", amount, i);
            }
        }
        Ok(())
    }

    /// Optimized yield calculation with caching
    pub fn calculate_yield_optimized(
        user_claim: &crate::state::UserClaim,
        cache: &mut Cache,
    ) -> u64 {
        let cache_key = format!("yield_{}_{}", user_claim.user, user_claim.locked_amount);
        let timestamp = cache.timestamp;
        
        cache.get_or_compute(&cache_key, || {
            crate::utils::CalculationUtils::calculate_yield(user_claim, timestamp)
        })
    }

    /// Batch state updates to reduce transaction overhead
    pub fn batch_state_update<F>(updates: Vec<F>) -> Result<()>
    where
        F: FnOnce() -> Result<()>,
    {
        for update_fn in updates {
            update_fn()?;
        }
        Ok(())
    }

    /// Early return optimization for validation
    pub fn validate_early_return<F>(condition: bool, error_fn: F) -> Result<()>
    where
        F: FnOnce() -> anchor_lang::error::ErrorCode,
    {
        if !condition {
            return Err(error_fn().into());
        }
        Ok(())
    }
}
