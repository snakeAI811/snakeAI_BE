use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub user_agent: String,
    pub ip_address: IpAddr,
    pub created_at: DateTime<Utc>,
    pub expired_at: DateTime<Utc>,
}
