use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Reward {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tweet_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub available: bool,
    pub message_sent: bool,
    // solana transaction
    pub transaction_signature: Option<String>,
    pub reward_amount: i64,
    pub wallet_address: Option<String>,
    pub block_time: Option<DateTime<Utc>>,
    // media data
    pub media_id: Option<String>,
    pub media_id_expires_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct RewardToReply {
    pub id: Uuid,
    pub tweet_id: String,
    // media data
    pub media_id: Option<String>,
    pub media_id_expires_at: Option<DateTime<Utc>>,
}

impl RewardToReply {
    pub fn media_available(&self) -> bool {
        if let Some(media_id_expires_at) = self.media_id_expires_at {
            return media_id_expires_at > Utc::now();
        }
        return false;
    }
}
