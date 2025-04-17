use anchor_client::solana_sdk::pubkey::Pubkey;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct User {
    pub id: Uuid,
    pub twitter_id: String,
    pub twitter_username: Option<String>,
    pub wallet_address: Option<String>,
    pub latest_claim_timestamp: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn wallet(&self) -> Option<Pubkey> {
        self.wallet_address
            .as_ref()
            .and_then(|addr| Pubkey::from_str(addr).ok())
    }
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
pub struct Profile {
    pub twitter_username: String,
    pub wallet_address: String,
    pub latest_claim_timestamp: Option<DateTime<Utc>>,
    pub reward_balance: i64,
    pub tweets: i64,
    pub likes: i64,
    pub replies: i64,
}
