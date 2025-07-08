use anchor_lang::prelude::*;
use crate::state::UserRole;
use crate::instructions::otc_swap_enhanced::SwapType;
use crate::state::VestingRoleType;

#[event(discriminator = b"poolinit")]
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

#[event(discriminator = b"claim")]
pub struct ClaimedReward {
    pub user: Pubkey,
    pub reward_amount: u64,
    pub burn_amount: u64,
    pub reward_level: u8,
}

// Patron-related events
#[event]
pub struct PatronApplicationSubmitted {
    pub user: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct PatronApproved {
    pub user: Pubkey,
    pub approved_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct PatronRevoked {
    pub user: Pubkey,
    pub revoked_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct PatronExited {
    pub user: Pubkey,
    pub burn_amount: u64,
    pub remaining_amount: u64,
    pub timestamp: i64,
}

// Lock and staking events
#[event]
pub struct TokensLocked {
    pub user: Pubkey,
    pub amount: u64,
    pub duration_months: u8,
    pub lock_start: i64,
    pub lock_end: i64,
    pub role: UserRole,
}

#[event]
pub struct TokensUnlocked {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct YieldClaimed {
    pub user: Pubkey,
    pub yield_amount: u64,
    pub timestamp: i64,
}

// DAO events
#[event]
pub struct DAOEligibilityAcquired {
    pub user: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct DAOSeatAcquired {
    pub user: Pubkey,
    pub seat_number: u32,
    pub timestamp: i64,
}

#[event]
pub struct DAOSeatTransferred {
    pub from: Pubkey,
    pub to: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct DAOSeatReturned {
    pub user: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct DAORegistryInitialized {
    pub total_seats: u32,
    pub min_stake: u64,
}

// ========== OTC SWAP EVENTS ==========

#[event]
pub struct SwapInitiated {
    pub seller: Pubkey,
    pub buyer_role_required: UserRole,
    pub otc_swap: Pubkey,
    pub token_amount: u64,
    pub sol_rate: u64,
    pub buyer_rebate: u64,
}

#[event]
pub struct SwapCompleted {
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub otc_swap: Pubkey,
    pub token_amount: u64,
    pub sol_payment: u64,
    pub rebate_amount: u64,
}

#[event]
pub struct SwapCancelled {
    pub seller: Pubkey,
    pub otc_swap: Pubkey,
}

#[event]
pub struct TokensBurned {
    pub user: Pubkey,
    pub amount: u64,
    pub reason: String,
}

// ========== MILESTONE 3: GOVERNANCE EVENTS ==========

#[event]
pub struct ProposalCreated {
    pub proposal_id: u64,
    pub proposer: Pubkey,
    pub title: String,
    pub description: String,
    pub proposal_type: crate::state::ProposalType,
    pub voting_ends_at: i64,
}

#[event]
pub struct VoteCast {
    pub proposal_id: u64,
    pub voter: Pubkey,
    pub vote_for: bool,
    pub voting_power: u32,
}

#[event]
pub struct ProposalFinalized {
    pub proposal_id: u64,
    pub status: crate::state::ProposalStatus,
    pub votes_for: u32,
    pub votes_against: u32,
    pub quorum_reached: bool,
}

#[event]
pub struct ProposalExecuted {
    pub proposal_id: u64,
    pub executor: Pubkey,
    pub executed_at: i64,
}

#[event]
pub struct ProposalCancelled {
    pub proposal_id: u64,
    pub canceller: Pubkey,
    pub cancelled_at: i64,
}

// ========== VESTING EVENTS ==========

#[event]
pub struct VestingCreated {
    pub user: Pubkey,
    pub amount: u64,
    pub role_type: VestingRoleType,
    pub start_timestamp: i64,
    pub unlock_timestamp: i64,
}

#[event]
pub struct VestingWithdrawn {
    pub user: Pubkey,
    pub amount: u64,
    pub yield_amount: u64,
    pub total_withdrawal: u64,
}

// ========== ENHANCED OTC SWAP EVENTS ==========

#[event]
pub struct EnhancedSwapCreated {
    pub seller: Pubkey,
    pub swap_account: Pubkey,
    pub token_amount: u64,
    pub price_per_token: u64,
    pub swap_type: SwapType,
    pub patron_rebate_percentage: u64,
    pub expires_at: i64,
    pub whitelisted_buyers: Vec<Pubkey>,
}

#[event]
pub struct EnhancedSwapCompleted {
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub swap_account: Pubkey,
    pub token_amount: u64,
    pub total_cost: u64,
    pub patron_rebate: u64,
    pub net_payment: u64,
}

#[event]
pub struct PatronRebateDistributed {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub rebate_amount: u64,
    pub swap_account: Pubkey,
}

// ========== STUB OTC SWAP/BURN EVENTS ==========

#[event]
pub struct PatronExitTracked {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct DAOEligibilityRevoked {
    pub user: Pubkey,
    pub reason: String,
}

