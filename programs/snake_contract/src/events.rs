use anchor_lang::prelude::*;

#[event]
pub struct RewardPoolInitialized {
    pub owner: Pubkey,
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub treasury: Pubkey,
}

#[event]
pub struct UserClaimInitialized {
    pub user: Pubkey,
}

#[event]
pub struct ClaimedReward {
    pub user: Pubkey,
    pub reward_amount: u64,
    pub burn_amount: u64,
    pub reward_level: u8,
}
