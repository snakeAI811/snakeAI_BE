use anchor_lang::prelude::*;

#[account]
pub struct OtcSwap {
    pub exiter: Pubkey,
    pub patron: Option<Pubkey>,
    pub amount: u64,
    pub rate: u64, // fixed rate
    pub is_active: bool,
}
