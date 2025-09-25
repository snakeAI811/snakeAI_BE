use crate::DatabasePool;
use sqlx::{query, query_as, Row, types::{chrono::{DateTime, Utc}, Uuid}};
use std::sync::Arc;
use types::{
    error::DbError,
    model::{OtcSwap, CreateOtcSwap, UpdateOtcSwap, OtcSwapWithUsers},
};

#[derive(Clone)]
pub struct OtcSwapRepository {
    pool: Arc<DatabasePool>,
}

impl OtcSwapRepository {
    pub fn new(pool: Arc<DatabasePool>) -> Self {
        Self { pool }
    }

    /// Create a new OTC swap
    pub async fn create(&self, create_swap: CreateOtcSwap) -> Result<OtcSwap, DbError> {
        let row = query!(
            r#"
            INSERT INTO otc_swaps (
                seller_id, seller_wallet, otc_swap_pda, token_amount, 
                sol_rate, buyer_rebate, swap_type, buyer_role_required, 
                initiate_tx_signature
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING 
                id, seller_id, buyer_id, seller_wallet, buyer_wallet,
                otc_swap_pda, token_amount, sol_rate, buyer_rebate,
                swap_type, buyer_role_required, status,
                initiate_tx_signature, accept_tx_signature, cancel_tx_signature,
                created_at, updated_at, completed_at, cancelled_at, expires_at
            "#,
            create_swap.seller_id,
            create_swap.seller_wallet,
            create_swap.otc_swap_pda,
            create_swap.token_amount,
            create_swap.sol_rate,
            create_swap.buyer_rebate,
            create_swap.swap_type,
            create_swap.buyer_role_required,
            create_swap.initiate_tx_signature
        )
        .fetch_one(self.pool.get_pool())
        .await?;

        Ok(OtcSwap {
            id: row.id,
            seller_id: row.seller_id,
            buyer_id: row.buyer_id,
            seller_wallet: row.seller_wallet,
            buyer_wallet: row.buyer_wallet,
            otc_swap_pda: row.otc_swap_pda,
            token_amount: row.token_amount,
            sol_rate: row.sol_rate,
            buyer_rebate: row.buyer_rebate.unwrap_or(0),
            swap_type: row.swap_type,
            buyer_role_required: row.buyer_role_required.unwrap_or_else(|| "none".to_string()),
            status: row.status,
            initiate_tx_signature: row.initiate_tx_signature,
            accept_tx_signature: row.accept_tx_signature,
            cancel_tx_signature: row.cancel_tx_signature,
            created_at: row.created_at,
            updated_at: row.updated_at,
            completed_at: row.completed_at,
            cancelled_at: row.cancelled_at,
            expires_at: row.expires_at,
        })
    }

    /// Get OTC swap by ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<OtcSwap>, DbError> {
        let row = query!(
            r#"
            SELECT 
                id, seller_id, buyer_id, seller_wallet, buyer_wallet,
                otc_swap_pda, token_amount, sol_rate, buyer_rebate,
                swap_type, buyer_role_required, status,
                initiate_tx_signature, accept_tx_signature, cancel_tx_signature,
                created_at, updated_at, completed_at, cancelled_at, expires_at
            FROM otc_swaps WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.pool.get_pool())
        .await?;

        Ok(row.map(|row| OtcSwap {
            id: row.id,
            seller_id: row.seller_id,
            buyer_id: row.buyer_id,
            seller_wallet: row.seller_wallet,
            buyer_wallet: row.buyer_wallet,
            otc_swap_pda: row.otc_swap_pda,
            token_amount: row.token_amount,
            sol_rate: row.sol_rate,
            buyer_rebate: row.buyer_rebate.unwrap_or(0),
            swap_type: row.swap_type,
            buyer_role_required: row.buyer_role_required.unwrap_or_else(|| "none".to_string()),
            status: row.status,
            initiate_tx_signature: row.initiate_tx_signature,
            accept_tx_signature: row.accept_tx_signature,
            cancel_tx_signature: row.cancel_tx_signature,
            created_at: row.created_at,
            updated_at: row.updated_at,
            completed_at: row.completed_at,
            cancelled_at: row.cancelled_at,
            expires_at: row.expires_at,
        }))
    }

    /// Get OTC swap by PDA
    pub async fn get_by_pda(&self, pda: &str) -> Result<Option<OtcSwap>, DbError> {
        let row = query!(
            r#"
            SELECT 
                id, seller_id, buyer_id, seller_wallet, buyer_wallet,
                otc_swap_pda, token_amount, sol_rate, buyer_rebate,
                swap_type, buyer_role_required, status,
                initiate_tx_signature, accept_tx_signature, cancel_tx_signature,
                created_at, updated_at, completed_at, cancelled_at, expires_at
            FROM otc_swaps WHERE otc_swap_pda = $1
            "#,
            pda
        )
        .fetch_optional(self.pool.get_pool())
        .await?;

        Ok(row.map(|row| OtcSwap {
            id: row.id,
            seller_id: row.seller_id,
            buyer_id: row.buyer_id,
            seller_wallet: row.seller_wallet,
            buyer_wallet: row.buyer_wallet,
            otc_swap_pda: row.otc_swap_pda,
            token_amount: row.token_amount,
            sol_rate: row.sol_rate,
            buyer_rebate: row.buyer_rebate.unwrap_or(0),
            swap_type: row.swap_type,
            buyer_role_required: row.buyer_role_required.unwrap_or_else(|| "none".to_string()),
            status: row.status,
            initiate_tx_signature: row.initiate_tx_signature,
            accept_tx_signature: row.accept_tx_signature,
            cancel_tx_signature: row.cancel_tx_signature,
            created_at: row.created_at,
            updated_at: row.updated_at,
            completed_at: row.completed_at,
            cancelled_at: row.cancelled_at,
            expires_at: row.expires_at,
        }))
    }

    /// Get active OTC swap by seller wallet
    pub async fn get_active_by_seller(&self, seller_wallet: &str) -> Result<Option<OtcSwap>, DbError> {
        // First, let's debug what swaps exist for this wallet
        let debug_swaps = query!(
            "SELECT id, status, expires_at, initiate_tx_signature FROM otc_swaps WHERE seller_wallet = $1 ORDER BY created_at DESC LIMIT 3",
            seller_wallet
        )
        .fetch_all(self.pool.get_pool())
        .await?;
        
        eprintln!("Debug: get_active_by_seller for wallet {}", seller_wallet);
        eprintln!("  Found {} total swaps:", debug_swaps.len());
        for swap in &debug_swaps {
            let now = Utc::now();
            eprintln!("    Swap {}: status={}, expires_at={}, expired={}, has_tx_sig={}", 
                swap.id, 
                swap.status, 
                swap.expires_at,
                swap.expires_at <= now,
                swap.initiate_tx_signature.is_some()
            );
        }

        let row = query!(
            r#"
            SELECT 
                id, seller_id, buyer_id, seller_wallet, buyer_wallet,
                otc_swap_pda, token_amount, sol_rate, buyer_rebate,
                swap_type, buyer_role_required, status,
                initiate_tx_signature, accept_tx_signature, cancel_tx_signature,
                created_at, updated_at, completed_at, cancelled_at, expires_at
            FROM otc_swaps 
            WHERE seller_wallet = $1 AND status = 'active' AND expires_at > NOW()
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            seller_wallet
        )
        .fetch_optional(self.pool.get_pool())
        .await?;

        Ok(row.map(|row| OtcSwap {
            id: row.id,
            seller_id: row.seller_id,
            buyer_id: row.buyer_id,
            seller_wallet: row.seller_wallet,
            buyer_wallet: row.buyer_wallet,
            otc_swap_pda: row.otc_swap_pda,
            token_amount: row.token_amount,
            sol_rate: row.sol_rate,
            buyer_rebate: row.buyer_rebate.unwrap_or(0),
            swap_type: row.swap_type,
            buyer_role_required: row.buyer_role_required.unwrap_or_else(|| "none".to_string()),
            status: row.status,
            initiate_tx_signature: row.initiate_tx_signature,
            accept_tx_signature: row.accept_tx_signature,
            cancel_tx_signature: row.cancel_tx_signature,
            created_at: row.created_at,
            updated_at: row.updated_at,
            completed_at: row.completed_at,
            cancelled_at: row.cancelled_at,
            expires_at: row.expires_at,
        }))
    }

    /// Update OTC swap
    pub async fn update(&self, id: Uuid, update_swap: UpdateOtcSwap) -> Result<Option<OtcSwap>, DbError> {
        let mut query_parts = Vec::new();
        let mut param_count = 1;

        if update_swap.buyer_id.is_some() {
            query_parts.push(format!("buyer_id = ${}", param_count));
            param_count += 1;
        }
        if update_swap.buyer_wallet.is_some() {
            query_parts.push(format!("buyer_wallet = ${}", param_count));
            param_count += 1;
        }
        if update_swap.status.is_some() {
            query_parts.push(format!("status = ${}", param_count));
            param_count += 1;
        }
        if update_swap.accept_tx_signature.is_some() {
            query_parts.push(format!("accept_tx_signature = ${}", param_count));
            param_count += 1;
        }
        if update_swap.cancel_tx_signature.is_some() {
            query_parts.push(format!("cancel_tx_signature = ${}", param_count));
            param_count += 1;
        }
        if update_swap.completed_at.is_some() {
            query_parts.push(format!("completed_at = ${}", param_count));
            param_count += 1;
        }
        if update_swap.cancelled_at.is_some() {
            query_parts.push(format!("cancelled_at = ${}", param_count));
            param_count += 1;
        }

        if query_parts.is_empty() {
            return self.get_by_id(id).await;
        }

        query_parts.push("updated_at = NOW()".to_string());
        let set_clause = query_parts.join(", ");
        let query_str = format!(
            r#"UPDATE otc_swaps SET {} WHERE id = ${} 
            RETURNING 
                id, seller_id, buyer_id, seller_wallet, buyer_wallet,
                otc_swap_pda, token_amount, sol_rate, buyer_rebate,
                swap_type, buyer_role_required, status,
                initiate_tx_signature, accept_tx_signature, cancel_tx_signature,
                created_at, updated_at, completed_at, cancelled_at, expires_at"#,
            set_clause, param_count
        );

        let mut query = sqlx::query(&query_str);
        
        if let Some(buyer_id) = update_swap.buyer_id {
            query = query.bind(buyer_id);
        }
        if let Some(buyer_wallet) = update_swap.buyer_wallet {
            query = query.bind(buyer_wallet);
        }
        if let Some(status) = update_swap.status {
            query = query.bind(status);
        }
        if let Some(accept_tx) = update_swap.accept_tx_signature {
            query = query.bind(accept_tx);
        }
        if let Some(cancel_tx) = update_swap.cancel_tx_signature {
            query = query.bind(cancel_tx);
        }
        if let Some(completed_at) = update_swap.completed_at {
            query = query.bind(completed_at);
        }
        if let Some(cancelled_at) = update_swap.cancelled_at {
            query = query.bind(cancelled_at);
        }
        
        query = query.bind(id);

        let row = query.fetch_optional(self.pool.get_pool()).await?;
        
        Ok(row.map(|row| {
            let id: Uuid = row.get("id");
            let seller_id: Uuid = row.get("seller_id");
            let buyer_id: Option<Uuid> = row.get("buyer_id");
            let seller_wallet: String = row.get("seller_wallet");
            let buyer_wallet: Option<String> = row.get("buyer_wallet");
            let otc_swap_pda: String = row.get("otc_swap_pda");
            let token_amount: i64 = row.get("token_amount");
            let sol_rate: i64 = row.get("sol_rate");
            let buyer_rebate: Option<i64> = row.get("buyer_rebate");
            let swap_type: String = row.get("swap_type");
            let buyer_role_required: Option<String> = row.get("buyer_role_required");
            let status: String = row.get("status");
            let initiate_tx_signature: Option<String> = row.get("initiate_tx_signature");
            let accept_tx_signature: Option<String> = row.get("accept_tx_signature");
            let cancel_tx_signature: Option<String> = row.get("cancel_tx_signature");
            let created_at: DateTime<Utc> = row.get("created_at");
            let updated_at: Option<DateTime<Utc>> = row.get("updated_at");
            let completed_at: Option<DateTime<Utc>> = row.get("completed_at");
            let cancelled_at: Option<DateTime<Utc>> = row.get("cancelled_at");
            let expires_at: DateTime<Utc> = row.get("expires_at");

            OtcSwap {
                id,
                seller_id,
                buyer_id,
                seller_wallet,
                buyer_wallet,
                otc_swap_pda,
                token_amount,
                sol_rate,
                buyer_rebate: buyer_rebate.unwrap_or(0),
                swap_type,
                buyer_role_required: buyer_role_required.unwrap_or_else(|| "none".to_string()),
                status,
                initiate_tx_signature,
                accept_tx_signature,
                cancel_tx_signature,
                created_at,
                updated_at,
                completed_at,
                cancelled_at,
                expires_at,
            }
        }))
    }

    /// Get active swaps with pagination
    pub async fn get_active_swaps(&self, page: i32, per_page: i32) -> Result<(Vec<OtcSwapWithUsers>, i64), DbError> {
        let offset = (page - 1) * per_page;

        // Get total count
        let total_count: i64 = query!(
            "SELECT COUNT(*) as count FROM otc_swaps WHERE status = 'active' AND expires_at > NOW()"
        )
        .fetch_one(self.pool.get_pool())
        .await?
        .count
        .unwrap_or(0);

        // Get swaps with user info
        let swaps = query!(
            r#"
            SELECT 
                s.id, s.seller_id, s.buyer_id, s.seller_wallet, s.buyer_wallet,
                s.otc_swap_pda, s.token_amount, s.sol_rate, s.buyer_rebate,
                s.swap_type, s.buyer_role_required, s.status,
                s.initiate_tx_signature, s.accept_tx_signature, s.cancel_tx_signature,
                s.created_at, s.updated_at, s.completed_at, s.cancelled_at, s.expires_at,
                seller.twitter_username as seller_username,
                buyer.twitter_username as buyer_username
            FROM otc_swaps s
            LEFT JOIN users seller ON s.seller_id = seller.id
            LEFT JOIN users buyer ON s.buyer_id = buyer.id
            WHERE s.status = 'active' AND s.expires_at > NOW()
            ORDER BY s.created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            per_page as i64,
            offset as i64
        )
        .fetch_all(self.pool.get_pool())
        .await?;

        let swap_results: Vec<OtcSwapWithUsers> = swaps
            .into_iter()
            .map(|row| OtcSwapWithUsers {
                swap: OtcSwap {
                    id: row.id,
                    seller_id: row.seller_id,
                    buyer_id: row.buyer_id,
                    seller_wallet: row.seller_wallet,
                    buyer_wallet: row.buyer_wallet,
                    otc_swap_pda: row.otc_swap_pda,
                    token_amount: row.token_amount,
                    sol_rate: row.sol_rate,
                    buyer_rebate: row.buyer_rebate.unwrap_or(0),
                    swap_type: row.swap_type,
                    buyer_role_required: row.buyer_role_required.unwrap_or_else(|| "none".to_string()),
                    status: row.status,
                    initiate_tx_signature: row.initiate_tx_signature,
                    accept_tx_signature: row.accept_tx_signature,
                    cancel_tx_signature: row.cancel_tx_signature,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    completed_at: row.completed_at,
                    cancelled_at: row.cancelled_at,
                    expires_at: row.expires_at,
                },
                seller_username: row.seller_username,
                buyer_username: row.buyer_username,
            })
            .collect();

        Ok((swap_results, total_count))
    }

    /// Get user's swaps
    pub async fn get_user_swaps(&self, user_id: Uuid) -> Result<Vec<OtcSwapWithUsers>, DbError> {
        let swaps = query!(
            r#"
            SELECT 
                s.id, s.seller_id, s.buyer_id, s.seller_wallet, s.buyer_wallet,
                s.otc_swap_pda, s.token_amount, s.sol_rate, s.buyer_rebate,
                s.swap_type, s.buyer_role_required, s.status,
                s.initiate_tx_signature, s.accept_tx_signature, s.cancel_tx_signature,
                s.created_at, s.updated_at, s.completed_at, s.cancelled_at, s.expires_at,
                seller.twitter_username as seller_username,
                buyer.twitter_username as buyer_username
            FROM otc_swaps s
            LEFT JOIN users seller ON s.seller_id = seller.id
            LEFT JOIN users buyer ON s.buyer_id = buyer.id
            WHERE s.seller_id = $1 OR s.buyer_id = $1
            ORDER BY s.created_at DESC
            "#,
            user_id
        )
        .fetch_all(self.pool.get_pool())
        .await?;

        let swap_results: Vec<OtcSwapWithUsers> = swaps
            .into_iter()
            .map(|row| OtcSwapWithUsers {
                swap: OtcSwap {
                    id: row.id,
                    seller_id: row.seller_id,
                    buyer_id: row.buyer_id,
                    seller_wallet: row.seller_wallet,
                    buyer_wallet: row.buyer_wallet,
                    otc_swap_pda: row.otc_swap_pda,
                    token_amount: row.token_amount,
                    sol_rate: row.sol_rate,
                    buyer_rebate: row.buyer_rebate.unwrap_or(0),
                    swap_type: row.swap_type,
                    buyer_role_required: row.buyer_role_required.unwrap_or_else(|| "none".to_string()),
                    status: row.status,
                    initiate_tx_signature: row.initiate_tx_signature,
                    accept_tx_signature: row.accept_tx_signature,
                    cancel_tx_signature: row.cancel_tx_signature,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    completed_at: row.completed_at,
                    cancelled_at: row.cancelled_at,
                    expires_at: row.expires_at,
                },
                seller_username: row.seller_username,
                buyer_username: row.buyer_username,
            })
            .collect();

        Ok(swap_results)
    }

    /// Get swap statistics
    pub async fn get_swap_stats(&self) -> Result<(i64, i64, i64, i64, i64, i64, i64), DbError> {
        let stats = query!(
            r#"
            SELECT 
                COUNT(*) as total_swaps,
                COUNT(*) FILTER (WHERE status = 'active' AND expires_at > NOW()) as active_swaps,
                COUNT(*) FILTER (WHERE status = 'completed') as completed_swaps,
                COUNT(*) FILTER (WHERE status = 'cancelled') as cancelled_swaps,
                COUNT(*) FILTER (WHERE status = 'active' AND expires_at <= NOW()) as expired_swaps,
                COALESCE(SUM(token_amount) FILTER (WHERE status = 'completed'), 0)::BIGINT as total_volume_tokens,
                COALESCE(SUM(token_amount * sol_rate) FILTER (WHERE status = 'completed'), 0)::BIGINT as total_volume_sol
            FROM otc_swaps
            "#
        )
        .fetch_one(self.pool.get_pool())
        .await?;
        
        Ok((
            stats.total_swaps.unwrap_or(0),
            stats.active_swaps.unwrap_or(0),
            stats.completed_swaps.unwrap_or(0),
            stats.cancelled_swaps.unwrap_or(0),
            stats.expired_swaps.unwrap_or(0),
            stats.total_volume_tokens.unwrap_or(0),
            stats.total_volume_sol.unwrap_or(0),
        ))
    }

    /// Mark expired swaps as expired
    pub async fn mark_expired_swaps(&self) -> Result<i64, DbError> {
        let result = query!(
            r#"
            UPDATE otc_swaps 
            SET status = 'expired', updated_at = NOW()
            WHERE status = 'active' AND expires_at <= NOW()
            "#
        )
        .execute(self.pool.get_pool())
        .await?;

        Ok(result.rows_affected() as i64)
    }

    /// Delete old completed/cancelled swaps (cleanup)
    pub async fn cleanup_old_swaps(&self, days_old: i32) -> Result<i64, DbError> {
        let result = query!(
            r#"
            DELETE FROM otc_swaps 
            WHERE status IN ('completed', 'cancelled', 'expired') 
            AND updated_at < NOW() - INTERVAL '1 day' * $1
            "#,
            days_old as f64
        )
        .execute(self.pool.get_pool())
        .await?;
        Ok(result.rows_affected() as i64)
    }

    /// Update initiate transaction signature for a swap
    pub async fn update_initiate_tx_signature(&self, swap_id: Uuid, seller_wallet: &str, tx_signature: String) -> Result<bool, DbError> {
        let result = query!(
            "UPDATE otc_swaps SET initiate_tx_signature = $1, updated_at = NOW() WHERE id = $2 AND seller_wallet = $3 AND initiate_tx_signature IS NULL",
            tx_signature,
            swap_id,
            seller_wallet
        )
        .execute(self.pool.get_pool())
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Get all swaps for a wallet (for debugging)
    pub async fn get_user_swaps_by_wallet(&self, wallet: &str) -> Result<Vec<OtcSwapWithUsers>, DbError> {
        let swaps = query!(
            r#"
            SELECT 
                s.id, s.seller_id, s.buyer_id, s.seller_wallet, s.buyer_wallet,
                s.otc_swap_pda, s.token_amount, s.sol_rate, s.buyer_rebate,
                s.swap_type, s.buyer_role_required, s.status,
                s.initiate_tx_signature, s.accept_tx_signature, s.cancel_tx_signature,
                s.created_at, s.updated_at, s.completed_at, s.cancelled_at, s.expires_at,
                seller.twitter_username as seller_username,
                buyer.twitter_username as buyer_username
            FROM otc_swaps s
            LEFT JOIN users seller ON s.seller_id = seller.id
            LEFT JOIN users buyer ON s.buyer_id = buyer.id
            WHERE s.seller_wallet = $1 OR s.buyer_wallet = $1
            ORDER BY s.created_at DESC
            "#,
            wallet
        )
        .fetch_all(self.pool.get_pool())
        .await?;

        let swap_results: Vec<OtcSwapWithUsers> = swaps
            .into_iter()
            .map(|row| OtcSwapWithUsers {
                swap: OtcSwap {
                    id: row.id,
                    seller_id: row.seller_id,
                    buyer_id: row.buyer_id,
                    seller_wallet: row.seller_wallet,
                    buyer_wallet: row.buyer_wallet,
                    otc_swap_pda: row.otc_swap_pda,
                    token_amount: row.token_amount,
                    sol_rate: row.sol_rate,
                    buyer_rebate: row.buyer_rebate.unwrap_or(0),
                    swap_type: row.swap_type,
                    buyer_role_required: row.buyer_role_required.unwrap_or_else(|| "none".to_string()),
                    status: row.status,
                    initiate_tx_signature: row.initiate_tx_signature,
                    accept_tx_signature: row.accept_tx_signature,
                    cancel_tx_signature: row.cancel_tx_signature,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    completed_at: row.completed_at,
                    cancelled_at: row.cancelled_at,
                    expires_at: row.expires_at,
                },
                seller_username: row.seller_username,
                buyer_username: row.buyer_username,
            })
            .collect();

        Ok(swap_results)
    }

    /// Get active OTC swap by seller wallet for cancellation (more lenient - doesn't require tx signature)
    pub async fn get_active_by_seller_for_cancel(&self, seller_wallet: &str) -> Result<Option<OtcSwap>, DbError> {
        eprintln!("Debug: get_active_by_seller_for_cancel for wallet {}", seller_wallet);
        
        let row = query!(
            r#"
            SELECT  *
            FROM otc_swaps 
            WHERE seller_wallet = $1 AND status = 'active'
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            seller_wallet
        )
        .fetch_optional(self.pool.get_pool())
        .await?;

        if let Some(ref row) = row {
            let now = Utc::now();
            eprintln!("  Found swap {}: status={}, expires_at={}, expired={}, has_tx_sig={}", 
                row.id, 
                row.status, 
                row.expires_at,
                row.expires_at <= now,
                row.initiate_tx_signature.is_some()
            );
        } else {
            eprintln!("  No active swap found for cancellation");
        }

        Ok(row.map(|row| OtcSwap {
            id: row.id,
            seller_id: row.seller_id,
            buyer_id: row.buyer_id,
            seller_wallet: row.seller_wallet,
            buyer_wallet: row.buyer_wallet,
            otc_swap_pda: row.otc_swap_pda,
            token_amount: row.token_amount,
            sol_rate: row.sol_rate,
            buyer_rebate: row.buyer_rebate.unwrap_or(0),
            swap_type: row.swap_type,
            buyer_role_required: row.buyer_role_required.unwrap_or_else(|| "none".to_string()),
            status: row.status,
            initiate_tx_signature: row.initiate_tx_signature,
            accept_tx_signature: row.accept_tx_signature,
            cancel_tx_signature: row.cancel_tx_signature,
            created_at: row.created_at,
            updated_at: row.updated_at,
            completed_at: row.completed_at,
            cancelled_at: row.cancelled_at,
            expires_at: row.expires_at,
        }))
    }
}