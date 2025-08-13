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


