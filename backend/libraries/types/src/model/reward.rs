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
pub struct RewardWithUserAndTweet {
    pub id: Uuid,
    // user data
    pub user_id: Uuid,
    pub twitter_id: String,
    pub twitter_username: Option<String>,
    // tweet_data
    pub tweet_id: Uuid,
    pub tweet_twitter_id: String,
    // reward_data
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

pub trait RewardUtils {
    fn media_id_expires_at(&self) -> Option<chrono::DateTime<Utc>>;
    fn id(&self) -> &Uuid;

    fn media_available(&self) -> bool {
        self.media_id_expires_at()
            .map_or(false, |expiry| expiry > Utc::now())
    }

    fn get_reward_url(&self, frontend_url: &str) -> String {
        format!("{}/claim/{}", frontend_url, self.id())
    }
}

impl RewardUtils for RewardToReply {
    fn media_id_expires_at(&self) -> Option<chrono::DateTime<Utc>> {
        self.media_id_expires_at
    }

    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl RewardUtils for Reward {
    fn media_id_expires_at(&self) -> Option<chrono::DateTime<Utc>> {
        self.media_id_expires_at
    }

    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl RewardUtils for RewardWithUserAndTweet {
    fn media_id_expires_at(&self) -> Option<chrono::DateTime<Utc>> {
        self.media_id_expires_at
    }

    fn id(&self) -> &Uuid {
        &self.id
    }
}
