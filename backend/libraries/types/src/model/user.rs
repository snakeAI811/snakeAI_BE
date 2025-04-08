use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct User {
    pub id: Uuid,
    pub twitter_id: String,
    pub twitter_username: String,
    pub wallet_address: String,
    pub latest_claim_timestamp: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
