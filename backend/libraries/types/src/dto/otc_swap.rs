use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateOtcSwapRequest {
    pub token_amount: u64,
    pub sol_rate: u64,
    pub buyer_rebate: u64,
    pub buyer_role_required: String, // "none", "staker", "patron"
    pub swap_type: Option<String>, // "exiter_to_patron", "patron_to_patron", "exiter_to_treasury"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateOtcSwapEnhancedRequest {
    pub token_amount: u64,
    pub sol_rate: u64,
    pub buyer_rebate: u64,
    pub swap_type: String, // "ExiterToPatron", "ExiterToTreasury", "PatronToPatron"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptOtcSwapRequest {
    pub seller_pubkey: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOtcSwapRequest {
    pub swap_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOtcSwapTxRequest {
    pub token_amount: u64,
    pub sol_rate: u64,
    pub buyer_rebate: u64,
    pub swap_type: String, // "ExiterToPatron", "ExiterToTreasury", "PatronToPatron"
    pub txSignature: String, // base64-encoded signed transaction
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtcSwapResponse {
    pub id: Uuid,
    pub seller_id: Uuid,
    pub buyer_id: Option<Uuid>,
    pub seller_wallet: String,
    pub buyer_wallet: Option<String>,
    pub seller_username: Option<String>,
    pub buyer_username: Option<String>,
    pub otc_swap_pda: String,
    pub token_amount: i64,
    pub sol_rate: i64,
    pub buyer_rebate: i64,
    pub swap_type: String,
    pub buyer_role_required: String,
    pub status: String,
    pub total_sol_payment: Option<u64>,
    pub net_sol_payment: Option<u64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
    pub is_expired: bool,
    pub can_accept: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveSwapsResponse {
    pub swaps: Vec<OtcSwapResponse>,
    pub total_count: i64,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MySwapsResponse {
    pub active_swaps: Vec<OtcSwapResponse>,
    pub completed_swaps: Vec<OtcSwapResponse>,
    pub cancelled_swaps: Vec<OtcSwapResponse>,
    pub total_active: i64,
    pub total_completed: i64,
    pub total_cancelled: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapStatsResponse {
    pub total_swaps: i64,
    pub active_swaps: i64,
    pub completed_swaps: i64,
    pub cancelled_swaps: i64,
    pub expired_swaps: i64,
    pub total_volume_tokens: i64,
    pub total_volume_sol: i64,
}