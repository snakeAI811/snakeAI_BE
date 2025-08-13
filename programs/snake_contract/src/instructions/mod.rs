pub mod initialize_reward_pool;
pub use initialize_reward_pool::*;

pub mod claim_reward;
pub use claim_reward::*;

pub mod update_reward_pool;
pub use update_reward_pool::*;

pub mod select_role;
pub use select_role::*;

pub mod claim_tokens_with_role;
pub use claim_tokens_with_role::*;

// pub mod otc_swap;
// pub use otc_swap::*;

// Enhanced OTC swap for Milestone 2
pub mod otc_swap_enhanced;
pub use otc_swap_enhanced::*;

// Vesting program
pub mod vesting;
pub use vesting::*;

// Stub OTC swap/burn logic
pub mod otc_swap_burn_stub;
pub use otc_swap_burn_stub::*;

pub mod sellback_to_project;
pub use sellback_to_project::*;

pub mod initialize_user_claim;
pub use initialize_user_claim::*;

// New patron-related instructions
pub mod patron_application;
pub use patron_application::*;

// Lock and staking instructions
pub mod lock_tokens;
pub use lock_tokens::*;

pub mod unlock_tokens;
pub use unlock_tokens::*;

pub mod claim_yield;
pub use claim_yield::*;

// New Patron Framework instructions
pub mod otc_trading;
pub use otc_trading::*;

pub mod patron_exit;
pub use patron_exit::*;

pub mod dao_governance;
pub use dao_governance::*;

// ========== NEW FEATURES: USER STATS & DASHBOARD ==========
pub mod update_user_stats;
pub use update_user_stats::*;

// ========== STAKING HISTORY TRACKING ==========
pub mod get_staking_history;
pub use get_staking_history::*;

