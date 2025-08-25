use crate::{repository::OtcSwapRepository, DatabasePool};
use sqlx::types::{Uuid, chrono::Utc};
use std::sync::Arc;
use types::{
    dto::{
        OtcSwapResponse, ActiveSwapsResponse, MySwapsResponse, SwapStatsResponse,
        InitiateOtcSwapRequest,
    },
    error::DbError,
    model::{CreateOtcSwap, UpdateOtcSwap, User},
};

// Custom trait for converting OtcSwap to OtcSwapWithUsers
trait IntoSwapWithUsers {
    fn into_swap_with_users(self) -> types::model::OtcSwapWithUsers;
}

impl IntoSwapWithUsers for types::model::OtcSwap {
    fn into_swap_with_users(self) -> types::model::OtcSwapWithUsers {
        types::model::OtcSwapWithUsers {
            swap: self,
            seller_username: None,
            buyer_username: None,
        }
    }
}

#[derive(Clone)]
pub struct OtcSwapService {
    repository: OtcSwapRepository,
}

impl OtcSwapService {
    pub fn new(pool: Arc<DatabasePool>) -> Self {
        Self {
            repository: OtcSwapRepository::new(pool),
        }
    }

    /// Create a new OTC swap
    pub async fn create_swap(
        &self,
        user: &User,
        request: &InitiateOtcSwapRequest,
        otc_swap_pda: String,
        tx_signature: Option<String>,
    ) -> Result<OtcSwapResponse, DbError> {
        let wallet = user.wallet_address
            .as_ref()
            .ok_or_else(|| DbError::ValidationError("User wallet not set".to_string()))?;

        // Check if user already has an active swap
        if let Some(_existing) = self.repository.get_active_by_seller(wallet).await? {
            return Err(DbError::ValidationError(
                "User already has an active OTC swap".to_string(),
            ));
        }

        let create_swap = CreateOtcSwap {
            seller_id: user.id,
            seller_wallet: wallet.clone(),
            otc_swap_pda,
            token_amount: request.token_amount as i64,
            sol_rate: request.sol_rate as i64,
            buyer_rebate: request.buyer_rebate as i64,
            swap_type: request.swap_type.clone().unwrap_or_else(|| "exiter_to_patron".to_string()),
            buyer_role_required: request.buyer_role_required.clone(),
            initiate_tx_signature: tx_signature,
        };

        let swap = self.repository.create(create_swap).await?;
        Ok(self.swap_to_response(swap.into_swap_with_users(), user.role.as_deref().unwrap_or("none")))
    }

    /// Create a new enhanced OTC swap
    pub async fn create_enhanced_swap(
        &self,
        user_id: Uuid,
        seller_wallet: &str,
        otc_swap_pda: &str,
        token_amount: i64,
        sol_rate: i64,
        buyer_rebate: i64,
        swap_type: &str,
    ) -> Result<(), DbError> {
        // Check if user already has an active swap
        if let Some(_existing) = self.repository.get_active_by_seller(seller_wallet).await? {
            return Err(DbError::ValidationError(
                "User already has an active OTC swap".to_string(),
            ));
        }

        // Convert swap_type to buyer_role_required for backwards compatibility
        let buyer_role_required = match swap_type {
            "ExiterToPatron" => "patron",
            "ExiterToTreasury" => "treasury", 
            "PatronToPatron" => "patron",
            _ => "none",
        };

        let create_swap = CreateOtcSwap {
            seller_id: user_id,
            seller_wallet: seller_wallet.to_string(),
            otc_swap_pda: otc_swap_pda.to_string(),
            token_amount,
            sol_rate,
            buyer_rebate,
            swap_type: swap_type.to_string(),
            buyer_role_required: buyer_role_required.to_string(),
            initiate_tx_signature: None,
        };

        self.repository.create(create_swap).await?;
        Ok(())
    }

    /// Accept an OTC swap
    pub async fn accept_swap(
        &self,
        buyer: &User,
        seller_wallet: &str,
        tx_signature: Option<String>,
    ) -> Result<OtcSwapResponse, DbError> {
        let buyer_wallet = buyer.wallet_address
            .as_ref()
            .ok_or_else(|| DbError::ValidationError("Buyer wallet not set".to_string()))?;

        // Get the active swap
        let swap = self.repository.get_active_by_seller(seller_wallet).await?
            .ok_or_else(|| DbError::NotFound("Active OTC swap not found".to_string()))?;

        // Check if swap can be accepted by this user
        let buyer_role = buyer.role.as_deref().unwrap_or("none");
        if !swap.can_be_accepted_by(buyer_role) {
            return Err(DbError::ValidationError(
                "User does not meet the requirements to accept this swap".to_string(),
            ));
        }

        // Update the swap
        let update_swap = UpdateOtcSwap {
            buyer_id: Some(buyer.id),
            buyer_wallet: Some(buyer_wallet.clone()),
            status: Some("completed".to_string()),
            accept_tx_signature: tx_signature,
            cancel_tx_signature: None,
            completed_at: Some(Utc::now()),
            cancelled_at: None,
        };

        let updated_swap = self.repository.update(swap.id, update_swap).await?
            .ok_or_else(|| DbError::NotFound("Swap not found after update".to_string()))?;

        Ok(self.swap_to_response(updated_swap.into_swap_with_users(), buyer_role))
    }

    /// Cancel an OTC swap
    pub async fn cancel_swap(
        &self,
        user: &User,
        tx_signature: Option<String>,
    ) -> Result<OtcSwapResponse, DbError> {
        let wallet = user.wallet_address
        .as_ref()
        .map(|w| w.trim())
        .ok_or_else(|| DbError::ValidationError("User wallet not set".to_string()))?;
        
        // Add debug logging
        eprintln!("Debug: Attempting to cancel swap for wallet: '{}'", wallet);
        eprintln!("Debug: Wallet length: {}", wallet.len());
        
        // Get the active swap (more lenient query for cancellation)
        let swap = self.repository.get_active_by_seller_for_cancel(wallet).await?
            .ok_or_else(|| {
                eprintln!("Debug: No active swap found for cancellation: {}", wallet);
                DbError::NotFound("Active OTC swap not found".to_string())
            })?;

        // Update the swap
        let update_swap = UpdateOtcSwap {
            buyer_id: None,
            buyer_wallet: None,
            status: Some("cancelled".to_string()),
            accept_tx_signature: None,
            cancel_tx_signature: tx_signature,
            completed_at: None,
            cancelled_at: Some(Utc::now()),
        };

        let updated_swap = self.repository.update(swap.id, update_swap).await?
            .ok_or_else(|| DbError::NotFound("Swap not found after update".to_string()))?;

        Ok(self.swap_to_response(updated_swap.into_swap_with_users(), user.role.as_deref().unwrap_or("none")))
    }

    /// Cancel an OTC swap by PDA (fallback when wallet lookup fails)
    pub async fn cancel_swap_by_pda(
        &self,
        otc_swap_pda: &str,
        tx_signature: Option<String>,
    ) -> Result<OtcSwapResponse, DbError> {
        // Find swap by PDA
        let swap = self
            .repository
            .get_by_pda(otc_swap_pda)
            .await?
            .ok_or_else(|| DbError::NotFound("OTC swap not found by PDA".to_string()))?;

        // Do not cancel completed swaps
        if swap.status == "completed" {
            return Err(DbError::ValidationError(
                "Cannot cancel a completed swap".to_string(),
            ));
        }

        // Update status to cancelled
        let update_swap = UpdateOtcSwap {
            buyer_id: None,
            buyer_wallet: None,
            status: Some("cancelled".to_string()),
            accept_tx_signature: None,
            cancel_tx_signature: tx_signature,
            completed_at: None,
            cancelled_at: Some(Utc::now()),
        };

        let updated_swap = self
            .repository
            .update(swap.id, update_swap)
            .await?
            .ok_or_else(|| DbError::NotFound("Swap not found after PDA cancel update".to_string()))?;

        Ok(self.swap_to_response(
            updated_swap.into_swap_with_users(),
            "none",
        ))
    }

    /// Get active swaps with pagination
    pub async fn get_active_swaps(
        &self,
        user_role: &str,
        page: i32,
        per_page: i32,
    ) -> Result<ActiveSwapsResponse, DbError> {
        let (swaps, total_count) = self.repository.get_active_swaps(page, per_page).await?;
        
        let swap_responses: Vec<OtcSwapResponse> = swaps
            .into_iter()
            .map(|swap_with_users| self.swap_with_users_to_response(swap_with_users, user_role))
            .collect();

        Ok(ActiveSwapsResponse {
            swaps: swap_responses,
            total_count,
            page,
            per_page,
        })
    }

    /// Get user's swaps
    pub async fn get_user_swaps(&self, user: &User) -> Result<MySwapsResponse, DbError> {
        let swaps = self.repository.get_user_swaps(user.id).await?;
        let user_role = user.role.as_deref().unwrap_or("none");

        let mut active_swaps = Vec::new();
        let mut completed_swaps = Vec::new();
        let mut cancelled_swaps = Vec::new();

        for swap_with_users in swaps {
            let response = self.swap_with_users_to_response(swap_with_users, user_role);
            match response.status.as_str() {
                "active" if !response.is_expired => active_swaps.push(response),
                "completed" => completed_swaps.push(response),
                "cancelled" | "expired" => cancelled_swaps.push(response),
                _ => {} // Handle other statuses if needed
            }
        }

        Ok(MySwapsResponse {
            total_active: active_swaps.len() as i64,
            total_completed: completed_swaps.len() as i64,
            total_cancelled: cancelled_swaps.len() as i64,
            active_swaps,
            completed_swaps,
            cancelled_swaps,
        })
    }

    /// Get swap statistics
    pub async fn get_swap_stats(&self) -> Result<SwapStatsResponse, DbError> {
        let (total, active, completed, cancelled, expired, volume_tokens, volume_sol) = 
            self.repository.get_swap_stats().await?;

        Ok(SwapStatsResponse {
            total_swaps: total,
            active_swaps: active,
            completed_swaps: completed,
            cancelled_swaps: cancelled,
            expired_swaps: expired,
            total_volume_tokens: volume_tokens,
            total_volume_sol: volume_sol,
        })
    }

    /// Mark expired swaps
    pub async fn mark_expired_swaps(&self) -> Result<i64, DbError> {
        self.repository.mark_expired_swaps().await
    }

    /// Get swap by PDA
    pub async fn get_swap_by_pda(&self, pda: &str, user_role: &str) -> Result<Option<OtcSwapResponse>, DbError> {
        if let Some(swap) = self.repository.get_by_pda(pda).await? {
            Ok(Some(self.swap_to_response(swap.into_swap_with_users(), user_role)))
        } else {
            Ok(None)
        }
    }

    /// Helper function to convert swap to response
    fn swap_to_response(&self, swap: types::model::OtcSwapWithUsers, user_role: &str) -> OtcSwapResponse {
        self.swap_with_users_to_response(swap, user_role)
    }

    /// Update swap with transaction signature
    pub async fn update_swap_tx_signature(
        &self,
        user: &User,
        tx_signature: String,
    ) -> Result<OtcSwapResponse, DbError> {
        let wallet = user.wallet_address
            .as_ref()
            .ok_or_else(|| DbError::ValidationError("User wallet not set".to_string()))?;

        // Find the user's active swap that doesn't have a transaction signature yet
        let swap = self.repository.get_active_by_seller(wallet).await?
            .ok_or_else(|| DbError::NotFound("No active OTC swap found".to_string()))?;

        // Check if the swap already has a transaction signature
        if swap.initiate_tx_signature.is_some() {
            return Err(DbError::ValidationError("OTC swap already has a transaction signature".to_string()));
        }

        // Update the initiate_tx_signature field
        let update_swap = UpdateOtcSwap {
            buyer_id: None,
            buyer_wallet: None,
            status: None,
            accept_tx_signature: None,
            cancel_tx_signature: None,
            completed_at: None,
            cancelled_at: None,
        };

        // Update the initiate transaction signature
        let success = self.repository.update_initiate_tx_signature(swap.id, wallet, tx_signature).await?;

        if !success {
            return Err(DbError::ValidationError("Could not update swap - it may already have a signature or not exist".to_string()));
        }

        // Return the updated swap
        let updated_swap = self.repository.get_by_id(swap.id).await?
            .ok_or_else(|| DbError::NotFound("Swap not found after update".to_string()))?;

        Ok(self.swap_to_response(updated_swap.into_swap_with_users(), user.role.as_deref().unwrap_or("none")))
    }

    /// Debug method to get all swaps for a wallet (for troubleshooting)
    pub async fn get_all_swaps_for_wallet(&self, wallet: &str) -> Result<Vec<types::model::OtcSwapWithUsers>, DbError> {
        let swaps = self.repository.get_user_swaps_by_wallet(wallet).await?;
        Ok(swaps)
    }

    /// Force cancel a swap by marking it as cancelled in database (for cleanup)
    pub async fn force_cancel_swap_by_wallet(&self, wallet: &str) -> Result<Option<OtcSwapResponse>, DbError> {
        // Find any active swap for this wallet
        let swap = self.repository.get_active_by_seller_for_cancel(wallet).await?;
        
        if let Some(swap) = swap {
            // Force update to cancelled status
            let update_swap = UpdateOtcSwap {
                buyer_id: None,
                buyer_wallet: None,
                status: Some("cancelled".to_string()),
                accept_tx_signature: None,
                cancel_tx_signature: Some("force_cancelled_by_admin".to_string()),
                completed_at: None,
                cancelled_at: Some(Utc::now()),
            };

            let updated_swap = self.repository.update(swap.id, update_swap).await?
                .ok_or_else(|| DbError::NotFound("Swap not found after update".to_string()))?;

            Ok(Some(self.swap_to_response(updated_swap.into_swap_with_users(), "none")))
        } else {
            Ok(None)
        }
    }

    /// Helper function to convert swap with users to response
    fn swap_with_users_to_response(&self, swap_with_users: types::model::OtcSwapWithUsers, user_role: &str) -> OtcSwapResponse {
        let swap = &swap_with_users.swap;
        let is_expired = swap.is_expired();
        let can_accept = swap.can_be_accepted_by(user_role);
        
        let total_sol_payment = swap.calculate_total_sol_payment().ok();
        let net_sol_payment = swap.calculate_net_payment().ok();

        OtcSwapResponse {
            id: swap.id,
            seller_id: swap.seller_id,
            buyer_id: swap.buyer_id,
            seller_wallet: swap.seller_wallet.clone(),
            buyer_wallet: swap.buyer_wallet.clone(),
            seller_username: swap_with_users.seller_username.clone(),
            buyer_username: swap_with_users.buyer_username.clone(),
            otc_swap_pda: swap.otc_swap_pda.clone(),
            token_amount: swap.token_amount,
            sol_rate: swap.sol_rate,
            buyer_rebate: swap.buyer_rebate,
            swap_type: swap.swap_type.clone(),
            buyer_role_required: swap.buyer_role_required.clone(),
            status: swap.status.clone(),
            total_sol_payment,
            net_sol_payment,
            created_at: swap.created_at,
            updated_at: swap.updated_at,
            completed_at: swap.completed_at,
            cancelled_at: swap.cancelled_at,
            expires_at: swap.expires_at,
            is_expired,
            can_accept,
        }
    }
}