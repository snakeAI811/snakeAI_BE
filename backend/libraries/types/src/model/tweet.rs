use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Tweet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tweet_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
pub struct TweetToInsert {
    pub user_id: Uuid,
    pub tweet_id: String,
    pub created_at: DateTime<Utc>,
}
