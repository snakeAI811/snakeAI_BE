pub mod auto_sync;
pub mod solana_sync;
pub mod mining;

pub use auto_sync::AutoSyncService;
pub use solana_sync::SolanaSync;
pub use mining::{MiningPhase, get_current_mining_phase, get_reward_burn_amount};