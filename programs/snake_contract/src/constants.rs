pub const REWARD_POOL_SEED: &[u8] = b"reward_pool";
pub const USER_CLAIM_SEED: &[u8] = b"user_claim";
pub const DAO_REGISTRY_SEED: &[u8] = b"dao_registry";
pub const OTC_SWAP_SEED: &[u8] = b"otc_swap";
pub const ESCROW_SEED: &[u8] = b"escrow";
pub const VESTING_SEED: &[u8] = b"vesting";
pub const LAMPORTS_PER_SNK: u64 = 1_000_000_000;

pub const STAKE_AMOUNT: u64 = 500_000_000;
pub const HIGH_REWARDS_THREADHOLD: u64 = 250_000_000;
pub const MEDIUM_REWARDS_THREADHOLD: u64 = 400_000_000;
pub const LOWER_REWARDS_THREADHOLD: u64 = STAKE_AMOUNT;

// Staking and locking constants
pub const STAKER_LOCK_DURATION_MONTHS: u8 = 3;
pub const PATRON_LOCK_DURATION_MONTHS: u8 = 6;
pub const STAKING_APY_PERCENT: u64 = 5; // 5% APY
pub const SECONDS_IN_YEAR: i64 = 365 * 24 * 60 * 60;

// DAO constants
pub const MIN_DAO_STAKE_AMOUNT: u64 = 250_000_000_000_000; // 250k tokens
pub const DAO_TOTAL_SEATS: u32 = 50;

// Patron exit penalty
pub const PATRON_EXIT_BURN_PERCENT: u64 = 20; // 20% burn on exit

// Patron qualification constants
pub const PATRON_MIN_TOKEN_AMOUNT: u64 = 250_000_000_000_000; // 250k tokens
pub const PATRON_MIN_WALLET_AGE_DAYS: u32 = 30; // 30 days minimum wallet age
pub const PATRON_MIN_STAKING_MONTHS: u8 = 6; // 6 months staking history required
pub const STAKER_MIN_STAKING_MONTHS: u8 = 3; // 3 months staking history required

// Time constants
pub const SECONDS_PER_MONTH: i64 = 30 * 24 * 60 * 60; // Approximate seconds in a month
pub const SIX_MONTHS_SECONDS: i64 = 6 * SECONDS_PER_MONTH;
