use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct SetWalletAddressRequest {
    pub wallet_address: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct GetClaimTransactionRequest {
    pub reward_id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct GetRewardsQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub available: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct GetTweetsQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
