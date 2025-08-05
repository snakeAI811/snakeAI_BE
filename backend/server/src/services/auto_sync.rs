use sqlx::types::Uuid;
use crate::state::AppState;
use super::SolanaSync;

/// Service to automatically sync Phase 1 data when rewards are created/updated
pub struct AutoSyncService {
    solana_sync: Option<SolanaSync>,
}

impl AutoSyncService {
    pub fn new() -> Self {
        Self {
            solana_sync: None,
        }
    }

    pub fn init_with_env(&mut self, state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
        // Only initialize in development/testing for now
        if !state.env.production {
            let solana_sync = SolanaSync::new(
                "https://api.devnet.solana.com", // Use devnet for now
                "[1,2,3,4,5]", // Placeholder - get from env
                "11111111111111111111111111111111", // Placeholder program ID
            )?;
            self.solana_sync = Some(solana_sync);
        }
        Ok(())
    }

    /// Called when a new Phase 1 reward is created
    pub async fn on_phase1_reward_created(
        &self,
        _state: &AppState,
        _user_id: &Uuid,
        _wallet_address: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // For now, we'll just log this event
        // In the future, this could trigger an immediate sync
        println!("📝 Phase 1 reward created - consider syncing to smart contract");
        
        // TODO: Implement actual sync logic here
        // if let (Some(sync), Some(wallet)) = (&self.solana_sync, wallet_address) {
        //     let phase1_total = state.service.reward.get_phase1_mining_total(user_id).await?;
        //     sync.sync_phase1_mining_data(wallet, phase1_total).await?;
        // }
        
        Ok(())
    }
}

impl Default for AutoSyncService {
    fn default() -> Self {
        Self::new()
    }
}
