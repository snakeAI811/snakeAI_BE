use anchor_lang::prelude::*;
use crate::{
    state::{UserClaim, UserRole},
    events::{PatronExitTracked, TokensBurned, DAOEligibilityRevoked},
    errors::SnakeError,
};

/// OTC Swap tracking for patron exit monitoring
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
        self.total_swaps = self.total_swaps.checked_add(1)
            .ok_or(SnakeError::ArithmeticOverflow)?;
        self.total_volume = self.total_volume.checked_add(amount)
            .ok_or(SnakeError::ArithmeticOverflow)?;
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

/// Tracking utilities for OTC swaps
pub struct OtcSwapTracking;

impl OtcSwapTracking {
    /// Track a swap operation and handle patron exit logic
    pub fn track_swap_operation(
        tracker: &mut OtcSwapTracker,
        user_claim: &UserClaim,
        amount: u64,
        is_sale: bool,
    ) -> Result<()> {
        // Track the swap
        tracker.track_swap(amount)?;
        
        // If this is a sale and user is a patron, track exit
        if is_sale && user_claim.role == UserRole::Patron {
            tracker.mark_exit();
            
            emit!(PatronExitTracked {
                user: tracker.user,
                amount,
                timestamp: Clock::get()?.unix_timestamp,
            });
        }
        
        Ok(())
    }
    
    /// Apply burn penalty for patron exit
    pub fn apply_burn_penalty(
        tracker: &mut OtcSwapTracker,
        user_claim: &mut UserClaim,
        burn_amount: u64,
    ) -> Result<()> {
        if user_claim.role == UserRole::Patron && !tracker.burn_penalty_applied {
            tracker.apply_burn_penalty();
            user_claim.sold_early = true;
            
            emit!(TokensBurned {
                user: tracker.user,
                amount: burn_amount,
                reason: "Patron exit penalty".to_string(),
            });
        }
        
        Ok(())
    }
    
    /// Revoke DAO eligibility for patron exit
    pub fn revoke_dao_eligibility(
        tracker: &mut OtcSwapTracker,
        user_claim: &mut UserClaim,
    ) -> Result<()> {
        if user_claim.role == UserRole::Patron && !tracker.dao_eligibility_revoked {
            tracker.revoke_dao_eligibility();
            user_claim.dao_eligible = false;
            user_claim.dao_seat_holder = false;
            
            emit!(DAOEligibilityRevoked {
                user: tracker.user,
                reason: "Patron exit detected - DAO eligibility revoked".to_string(),
            });
        }
        
        Ok(())
    }
    
    /// Check if user has exceeded swap limits
    pub fn check_swap_limits(
        tracker: &OtcSwapTracker,
        max_swaps: u64,
        max_volume: u64,
    ) -> Result<()> {
        require!(
            tracker.total_swaps <= max_swaps,
            SnakeError::MaxOTCLimitExceeded
        );
        require!(
            tracker.total_volume <= max_volume,
            SnakeError::MaxOTCLimitExceeded
        );
        Ok(())
    }
}
