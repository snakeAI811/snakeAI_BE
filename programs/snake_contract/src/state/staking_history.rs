use anchor_lang::prelude::*;
use crate::state::UserRole;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace, Debug)]
pub enum StakingAction {
    Lock,
    Unlock,
    YieldClaim,
    RoleChange,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct StakingHistoryEntry {
    pub action: StakingAction,
    pub amount: u64,
    pub timestamp: i64,
    pub role: UserRole,
    pub lock_duration_months: u8,
    pub yield_amount: u64, // For yield claims
    #[max_len(32)]
    pub additional_data: String, // For extra context (max 32 chars)
}

#[account]
#[derive(InitSpace)]
pub struct UserStakingHistory {
    pub user: Pubkey,
    pub initialized: bool,
    pub total_entries: u32,
    pub total_locked: u64,
    pub total_unlocked: u64,
    pub total_yield_claimed: u64,
    pub first_stake_timestamp: i64,
    pub last_activity_timestamp: i64,
    #[max_len(50)] // Store last 50 entries
    pub entries: Vec<StakingHistoryEntry>,
}

impl UserStakingHistory {
    pub fn init(&mut self, user: Pubkey) {
        self.user = user;
        self.initialized = true;
        self.total_entries = 0;
        self.total_locked = 0;
        self.total_unlocked = 0;
        self.total_yield_claimed = 0;
        self.first_stake_timestamp = 0;
        self.last_activity_timestamp = 0;
        self.entries = Vec::new();
    }

    pub fn add_entry(&mut self, entry: StakingHistoryEntry) -> Result<()> {
        // Update totals based on action
        match entry.action {
            StakingAction::Lock => {
                self.total_locked = self.total_locked
                    .checked_add(entry.amount)
                    .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?;
                
                if self.first_stake_timestamp == 0 {
                    self.first_stake_timestamp = entry.timestamp;
                }
            },
            StakingAction::Unlock => {
                self.total_unlocked = self.total_unlocked
                    .checked_add(entry.amount)
                    .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?;
            },
            StakingAction::YieldClaim => {
                self.total_yield_claimed = self.total_yield_claimed
                    .checked_add(entry.yield_amount)
                    .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?;
            },
            StakingAction::RoleChange => {
                // No amount tracking for role changes
            }
        }

        // Store timestamp before moving entry
        let entry_timestamp = entry.timestamp;
        
        // Add entry to history (keep only last 50 entries)
        if self.entries.len() >= 50 {
            self.entries.remove(0); // Remove oldest entry
        }
        self.entries.push(entry);
        
        self.total_entries = self.total_entries
            .checked_add(1)
            .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?;
        
        self.last_activity_timestamp = entry_timestamp;
        
        Ok(())
    }

    pub fn get_staking_duration_days(&self, current_timestamp: i64) -> u32 {
        if self.first_stake_timestamp == 0 {
            return 0;
        }
        let duration_seconds = current_timestamp - self.first_stake_timestamp;
        (duration_seconds / (24 * 60 * 60)) as u32
    }

    pub fn get_recent_entries(&self, count: usize) -> Vec<StakingHistoryEntry> {
        let start_index = if self.entries.len() > count {
            self.entries.len() - count
        } else {
            0
        };
        self.entries[start_index..].to_vec()
    }

    pub fn has_sufficient_staking_history(&self, required_months: u8) -> bool {
        let current_time = Clock::get().unwrap().unix_timestamp;
        let required_seconds = (required_months as i64) * 30 * 24 * 60 * 60; // Approximate months
        
        if self.first_stake_timestamp == 0 {
            return false;
        }
        
        let staking_duration = current_time - self.first_stake_timestamp;
        staking_duration >= required_seconds
    }
}

#[account]
#[derive(InitSpace)]
pub struct GlobalStakingStats {
    pub initialized: bool,
    pub total_users: u32,
    pub total_stakers: u32,
    pub total_patrons: u32,
    pub total_locked_amount: u64,
    pub total_yield_distributed: u64,
    pub last_updated: i64,
}

impl GlobalStakingStats {
    pub fn init(&mut self) {
        self.initialized = true;
        self.total_users = 0;
        self.total_stakers = 0;
        self.total_patrons = 0;
        self.total_locked_amount = 0;
        self.total_yield_distributed = 0;
        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }

    pub fn update_user_count(&mut self, old_role: UserRole, new_role: UserRole) -> Result<()> {
        // Decrease old role count
        match old_role {
            UserRole::Staker => {
                self.total_stakers = self.total_stakers.saturating_sub(1);
            },
            UserRole::Patron => {
                self.total_patrons = self.total_patrons.saturating_sub(1);
            },
            UserRole::None => {
                // No change for None role
            }
        }

        // Increase new role count
        match new_role {
            UserRole::Staker => {
                self.total_stakers = self.total_stakers
                    .checked_add(1)
                    .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?;
            },
            UserRole::Patron => {
                self.total_patrons = self.total_patrons
                    .checked_add(1)
                    .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?;
            },
            UserRole::None => {
                // No change for None role
            }
        }

        self.last_updated = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }

    pub fn update_locked_amount(&mut self, amount_change: i64) -> Result<()> {
        if amount_change >= 0 {
            self.total_locked_amount = self.total_locked_amount
                .checked_add(amount_change as u64)
                .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?;
        } else {
            self.total_locked_amount = self.total_locked_amount
                .saturating_sub((-amount_change) as u64);
        }
        
        self.last_updated = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }

    pub fn add_yield_distributed(&mut self, yield_amount: u64) -> Result<()> {
        self.total_yield_distributed = self.total_yield_distributed
            .checked_add(yield_amount)
            .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?;
        
        self.last_updated = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }
}