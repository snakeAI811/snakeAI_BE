use anchor_lang::prelude::*;

use crate::events::RewardPoolInitialized;

#[account]
#[derive(Default, InitSpace)]
pub struct RewardPool {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub treasury: Pubkey,

    pub tweet_number: u64,
    pub minted_accum: u64,
    pub burned: u64,
    pub airdropped: u64,
}

impl RewardPool {
    pub fn init(&mut self, owner: Pubkey, mint: Pubkey, treasury: Pubkey) {
        self.owner = owner;
        self.mint = mint;
        self.treasury = treasury;

        self.tweet_number = 0;
        self.minted_accum = 0;
        self.burned = 0;
        self.airdropped = 0;

        emit!(RewardPoolInitialized {
            owner: self.owner,
            mint: self.mint,
            treasury: self.treasury
        });
    }
}
