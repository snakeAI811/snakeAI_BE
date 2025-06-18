use anchor_lang::prelude::*;

use crate::events::UserClaimInitialized;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Default, InitSpace)]
pub enum UserRole {
    #[default]
    None,
    Exit,
    Stake,
    Patron,
}

#[account]
#[derive(Default, InitSpace)]
pub struct UserClaim {
    pub initialized: bool,
    pub user: Pubkey,
    pub last_claim_timestamp: i64,
    pub role: UserRole, // New: user role
}

impl UserClaim {
    pub fn init(&mut self, user: Pubkey) {
        self.initialized = true;
        self.user = user;
        self.last_claim_timestamp = 0;
        self.role = UserRole::None;
        emit!(UserClaimInitialized { user: self.user });
    }
}
