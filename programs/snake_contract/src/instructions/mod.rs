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

pub mod otc_swap;
pub use otc_swap::*;

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

// // DAO instructions
// pub mod initialize_dao_registry;
// pub use initialize_dao_registry::*;

// pub mod acquire_dao_seat;
// pub use acquire_dao_seat::*;

// pub mod transfer_dao_seat;
// pub use transfer_dao_seat::*;

// pub mod return_dao_seat;
// pub use return_dao_seat::*;

// // View functions
// pub mod view_functions;
// pub use view_functions::*;

// // ========== MILESTONE 3: GOVERNANCE ==========
// pub mod governance;
// pub use governance::*;

// pub mod governance_views;
// pub use governance_views::*;

// // ========== NEW FEATURES: USER STATS & DASHBOARD ==========
// pub mod update_user_stats;
// pub use update_user_stats::*;

// pub mod dashboard_views;
// pub use dashboard_views::*;
