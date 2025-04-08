use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Reward {
    pub id: Uuid,
    pub user_id: Uuid,
    pub reward_amount: i64,
    pub tx_id: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
