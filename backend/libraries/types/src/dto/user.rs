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

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct GetTweetsQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct ClaimTweetRewardRequest {
    pub tweet_id: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct SubmitTweetRequest {
    pub tweet_id: String,
    pub content: String,
    pub hashtags: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TweetMiningStatusResponse {
    pub total_tweets: i64,
    pub phase1_count: i64,
    pub phase2_count: i64,
    pub pending_rewards: i64,
    pub total_rewards_claimed: i64,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct SetRewardFlagRequest {
    pub tweet_id: String,
}
