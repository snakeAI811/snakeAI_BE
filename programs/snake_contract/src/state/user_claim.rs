use anchor_lang::prelude::*;

use crate::events::UserClaimInitialized;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Default, InitSpace)]
pub enum UserRole {
    #[default]
    None,        // Normal user - can mine, claim, sell at TGE
    Staker,      // Locks tokens for 3 months, earns 5% APY
    Patron,      // Deep commitment - governance eligible
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Default, InitSpace)]
pub enum PatronStatus {
    #[default]
    None,
    Applied,
    Approved,
    Revoked,
}

#[account]
#[derive(Default, InitSpace)]
pub struct UserClaim {
    pub initialized: bool,
    pub user: Pubkey,
    pub last_claim_timestamp: i64,
    pub role: UserRole,
    
    // Patron-specific fields
    pub patron_status: PatronStatus,
    pub patron_application_timestamp: i64,
    pub patron_approval_timestamp: i64,
    
    // Lock and staking fields
    pub locked_amount: u64,
    pub lock_start_timestamp: i64,
    pub lock_end_timestamp: i64,
    pub lock_duration_months: u8, // 3 for stakers, 6 for patrons
    
    // Staking yield tracking
    pub last_yield_claim_timestamp: i64,
    pub total_yield_claimed: u64,
    
    // Commitment tracking
    pub sold_early: bool,
    pub mined_in_phase2: bool,
    
    // DAO eligibility
    pub dao_eligible: bool,
    pub dao_seat_holder: bool,
    pub dao_seat_acquired_timestamp: i64,
    
    // Patron qualification criteria (for your requirements)
    pub total_mined_phase1: u64,        // Total tokens mined in Phase 1
    pub wallet_age_days: u32,           // Wallet age in days (for patron selection)
    pub community_score: u32,           // Community contribution score (0-100)
    pub patron_qualification_score: u32, // Calculated eligibility score
}

impl UserClaim {
    pub fn init(&mut self, user: Pubkey) {
        self.initialized = true;
        self.user = user;
        self.last_claim_timestamp = 0;
        self.role = UserRole::None;
        self.patron_status = PatronStatus::None;
        self.patron_application_timestamp = 0;
        self.patron_approval_timestamp = 0;
        self.locked_amount = 0;
        self.lock_start_timestamp = 0;
        self.lock_end_timestamp = 0;
        self.lock_duration_months = 0;
        self.last_yield_claim_timestamp = 0;
        self.total_yield_claimed = 0;
        self.sold_early = false;
        self.mined_in_phase2 = false;
        self.dao_eligible = false;
        self.dao_seat_holder = false;
        self.dao_seat_acquired_timestamp = 0;
        self.total_mined_phase1 = 0;
        self.wallet_age_days = 0;
        self.community_score = 0;
        self.patron_qualification_score = 0;
        emit!(UserClaimInitialized { user: self.user });
    }
    
    pub fn is_locked(&self) -> bool {
        let current_time = Clock::get().unwrap().unix_timestamp;
        self.locked_amount > 0 && current_time < self.lock_end_timestamp
    }
    
    pub fn can_unlock(&self) -> bool {
        let current_time = Clock::get().unwrap().unix_timestamp;
        current_time >= self.lock_end_timestamp
    }
    
    pub fn calculate_yield(&self) -> Result<u64> {
        if self.role != UserRole::Staker || self.locked_amount == 0 {
            return Ok(0);
        }
        
        let current_time = Clock::get()?.unix_timestamp;
        let last_claim = if self.last_yield_claim_timestamp == 0 {
            self.lock_start_timestamp
        } else {
            self.last_yield_claim_timestamp
        };
        
        let time_diff = current_time.saturating_sub(last_claim);
        let seconds_in_year = 365 * 24 * 60 * 60;
        
        // 5% APY calculation
        let yield_amount = (self.locked_amount as u128)
            .checked_mul(5) // 5% APY
            .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?
            .checked_mul(time_diff as u128)
            .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?
            .checked_div(100) // percentage
            .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?
            .checked_div(seconds_in_year as u128)
            .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?;
        
        Ok(yield_amount as u64)
    }
    
    /// Calculate patron qualification score based on your requirements:
    /// - Wallet age / KYC (optional)
    /// - Contribution to community  
    /// - On-chain record (mining history)
    pub fn calculate_patron_qualification_score(&mut self) -> u32 {
        let mut score = 0u32;
        
        // Mining contribution (40 points max) - "On-chain record"
        if self.total_mined_phase1 >= 1_000_000_000 { // 1000 tokens
            score += 40;
        } else if self.total_mined_phase1 >= 500_000_000 { // 500 tokens
            score += 30;
        } else if self.total_mined_phase1 >= 100_000_000 { // 100 tokens
            score += 20;
        } else if self.total_mined_phase1 > 0 {
            score += 10;
        }
        
        // Wallet age (30 points max) - "Wallet age / KYC"
        if self.wallet_age_days >= 90 { // 3+ months old
            score += 30;
        } else if self.wallet_age_days >= 30 { // 1+ month old
            score += 20;
        } else if self.wallet_age_days >= 7 { // 1+ week old
            score += 10;
        }
        
        // Community contribution (30 points max) - "Contribution to community"
        score += self.community_score.min(30);
        
        // Update stored score
        self.patron_qualification_score = score;
        score
    }
    
    /// Check if user meets minimum patron qualification criteria
    pub fn meets_patron_criteria(&mut self, min_score: u32) -> bool {
        let score = self.calculate_patron_qualification_score();
        score >= min_score
    }
    
    /// Check if user meets Month 6 DAO eligibility per your requirements:
    /// "Enter DAO: Stake during Phase 2 for 3 months + Hold XX SNAKE at Month 6"
    pub fn check_month6_dao_eligibility(&self, current_balance: u64, min_dao_stake: u64) -> bool {
        let current_time = Clock::get().unwrap().unix_timestamp;
        
        match self.role {
            UserRole::Patron => {
                // For Patrons: must have completed 6-month commitment + mined Phase 2
                if self.patron_status == PatronStatus::Approved {
                    let six_months_seconds = 6 * 30 * 24 * 60 * 60;
                    let commitment_end = self.patron_approval_timestamp + six_months_seconds;
                    
                    current_time >= commitment_end && 
                    !self.sold_early && 
                    self.mined_in_phase2
                } else {
                    false
                }
            },
            UserRole::Staker => {
                // For Stakers: must have staked during Phase 2 for 3 months + hold minimum
                self.lock_duration_months >= 3 &&
                current_time >= self.lock_end_timestamp && // Completed 3-month lock
                current_balance >= min_dao_stake &&
                self.mined_in_phase2
            },
            UserRole::None => false, // Normal users not eligible
        }
    }
}
