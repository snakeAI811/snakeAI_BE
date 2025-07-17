use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SelectRoleRequest {
    pub role: String, // "staker", "patron", or "seller"
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PatronApplicationRequest {
    pub wallet_age_days: u32,
    pub community_score: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApprovePatronRequest {
    pub user_wallet: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RevokePatronRequest {
    pub user_wallet: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateVestingRequest {
    pub amount: u64,
    pub role_type: String, // "staker" or "patron"
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LockTokensRequest {
    pub amount: u64,
    pub duration_months: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClaimYieldRequest {
    pub expected_yield: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InitiateOtcSwapRequest {
    pub token_amount: u64,
    pub sol_rate: u64, // SOL per token in lamports
    pub buyer_rebate: u64, // basis points (100 = 1%)
    pub swap_type: String, // "NormalToPatron", "PatronToNormal", "NormalToNormal"
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExecuteOtcSwapRequest {
    pub swap_id: String, // Base58 encoded swap PDA
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CancelOtcSwapRequest {
    pub swap_id: String, // Base58 encoded swap PDA
}
