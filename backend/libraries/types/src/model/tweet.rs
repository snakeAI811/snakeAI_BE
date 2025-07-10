use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Tweet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tweet_id: String,
    pub created_at: DateTime<Utc>,
    pub mining_phase: Option<i16>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct TweetWithUser {
    pub id: Uuid,
    pub twitter_id: String,
    pub twitter_username: Option<String>,
    pub tweet_id: String,
    pub created_at: DateTime<Utc>,
    pub mining_phase: Option<i16>,
}
