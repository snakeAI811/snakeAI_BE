use database::{service::Services, pool::DatabasePool};
use std::sync::Arc;
use tokio::time::{Duration, interval};
use crate::solana_sync::SolanaSync;

pub struct Phase1SyncJob {
    services: Services,
    solana_sync: SolanaSync,
}

impl Phase1SyncJob {
    pub fn new(db_conn: &Arc<DatabasePool>, solana_sync: SolanaSync) -> Self {
        Self {
            services: Services::init(db_conn),
            solana_sync,
        }
    }

    /// Run the job once to sync all users who have Phase 1 mining but aren't synced to contract yet
    pub async fn run_once(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Starting Phase 1 sync job...");

        // Get all users with wallet addresses
        let users = self.services.user.get_all_users_with_wallets().await?;

        let mut synced_count = 0;
        let mut skipped_count = 0;

        for user in users {
            if let Some(wallet_address) = &user.wallet_address {
                // Get Phase 1 mining total from database
                let phase1_total = self
                    .services
                    .reward
                    .get_phase1_mining_total(&user.id)
                    .await
                    .unwrap_or(0);

                if phase1_total > 0 {
                    // TODO: Check if user is already synced to avoid duplicate transactions
                    // For now, sync everyone with Phase 1 mining
                    match self.solana_sync.sync_phase1_mining_data(wallet_address, phase1_total).await {
                        Ok(tx_sig) => {
                            synced_count += 1;
                            println!("✅ Synced Phase 1 data for user {}: {} tokens (tx: {})", 
                                wallet_address, phase1_total, tx_sig);
                        }
                        Err(e) => {
                            println!("❌ Failed to sync user {}: {}", wallet_address, e);
                        }
                    }
                } else {
                    skipped_count += 1;
                }
            }
        }

        println!("📊 Phase 1 sync job completed: {} synced, {} skipped", synced_count, skipped_count);
        Ok(())
    }

    /// Run the job periodically (every hour)
    pub async fn run_scheduled(&self) {
        let mut interval = interval(Duration::from_secs(3600)); // 1 hour

        loop {
            interval.tick().await;
            
            if let Err(e) = self.run_once().await {
                eprintln!("❌ Phase 1 sync job failed: {}", e);
            }
        }
    }
}

// Re-export the SolanaSync from server for convenience
pub use server::services::SolanaSync;

// Create a mock module since we don't have the actual server implementation
mod solana_sync {
    use std::str::FromStr;

    pub struct SolanaSync;

    impl SolanaSync {
        pub fn new(_rpc_url: &str, _admin_key: &str, _program_id: &str) -> Result<Self, Box<dyn std::error::Error>> {
            Ok(Self)
        }

        pub async fn sync_phase1_mining_data(&self, _wallet: &str, _amount: i64) -> Result<String, Box<dyn std::error::Error>> {
            // Mock implementation - replace with actual sync
            Ok("mock_transaction_signature".to_string())
        }
    }
}
