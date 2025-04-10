use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct SetWalletAddressRequest {
    pub wallet_address: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct GetRewardsQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub available: Option<bool>,
}
