use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum VestingRoleType {
    Staker = 1,  // 3-month lock with 5% APY
    Patron = 2,  // 6-month lock with no-sell rule
}

#[account]
#[derive(Default, InitSpace)]
pub struct VestingAccount {
    pub user: Pubkey,
    pub amount: u64,
    pub role_type: VestingRoleType,
    pub start_timestamp: i64,
    pub unlock_timestamp: i64,
    pub withdrawn: bool,
    pub early_exit_flag: bool,  // Track early exits for Patrons
    pub bump: u8,
}

impl VestingAccount {
    pub fn init(
        &mut self,
        user: Pubkey,
        amount: u64,
        role_type: VestingRoleType,
        bump: u8,
    ) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        
        self.user = user;
        self.amount = amount;
        self.role_type = role_type;
        self.start_timestamp = current_time;
        self.withdrawn = false;
        self.early_exit_flag = false;
        self.bump = bump;
        
        // Set unlock timestamp based on role type
        match role_type {
            VestingRoleType::Staker => {
                // 3 months for stakers
                self.unlock_timestamp = current_time + (3 * 30 * 24 * 60 * 60);
            },
            VestingRoleType::Patron => {
                // 6 months for patrons
                self.unlock_timestamp = current_time + (6 * 30 * 24 * 60 * 60);
            },
        }
        
        Ok(())
    }
    
    pub fn is_unlocked(&self) -> bool {
        let current_time = Clock::get().unwrap().unix_timestamp;
        current_time >= self.unlock_timestamp
    }
    
    pub fn can_withdraw(&self) -> bool {
        !self.withdrawn && self.is_unlocked()
    }
    
    pub fn calculate_yield(&self) -> Result<u64> {
        // Only stakers earn yield
        if self.role_type != VestingRoleType::Staker {
            return Ok(0);
        }
        
        let current_time = Clock::get()?.unix_timestamp;
        let time_locked = if self.is_unlocked() {
            self.unlock_timestamp - self.start_timestamp
        } else {
            current_time - self.start_timestamp
        };
        
        // 5% APY calculation
        let seconds_in_year = 365 * 24 * 60 * 60;
        let yield_amount = (self.amount as u128)
            .checked_mul(5) // 5% APY
            .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?
            .checked_mul(time_locked as u128)
            .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?
            .checked_div(100) // percentage
            .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?
            .checked_div(seconds_in_year as u128)
            .ok_or(crate::errors::SnakeError::ArithmeticOverflow)?;
        
        Ok(yield_amount as u64)
    }
    
    pub fn mark_early_exit(&mut self) {
        if self.role_type == VestingRoleType::Patron {
            self.early_exit_flag = true;
        }
    }
}

impl Default for VestingRoleType {
    fn default() -> Self {
        VestingRoleType::Staker
    }
}