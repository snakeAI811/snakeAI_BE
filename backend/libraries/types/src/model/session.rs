use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub user_agent: String,
    pub ip_address: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}
