use anchor_lang::prelude::*;

use crate::events::UserClaimInitialized;

#[account]
#[derive(Default, InitSpace)]
pub struct UserClaim {
    pub initialized: bool,
    pub user: Pubkey,
    pub last_claim_timestamp: i64,
}

impl UserClaim {
    pub fn init(&mut self, user: Pubkey) {
        self.initialized = true;
        self.user = user;
        self.last_claim_timestamp = 0;

        emit!(UserClaimInitialized { user: self.user });
    }
}
