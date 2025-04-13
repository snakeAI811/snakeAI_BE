use crate::pool::DatabasePool;
use sqlx::types::{chrono::Utc, Uuid};
use std::{sync::Arc, time::Duration};
use types::model::Session;

#[derive(Clone)]
pub struct SessionRepository {
    db_conn: Arc<DatabasePool>,
}

impl SessionRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn create_session(
        &self,
        user_id: &Uuid,
        user_agent: &str,
        ip_address: &str,
        session_ttl_in_minutes: u64,
    ) -> Result<Session, sqlx::Error> {
        let created_at = Utc::now();
        let expires_at =
            created_at + Duration::from_secs(session_ttl_in_minutes.checked_mul(60).unwrap());
        let session = sqlx::query_as!(
            Session,
            r#"
                INSERT INTO sessions (user_id, user_agent, ip_address, created_at, expires_at)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING *
            "#,
            user_id,
            user_agent,
            ip_address,
            created_at,
            expires_at
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(session)
    }

    pub async fn get_session_by_session_id(
        &self,
        session_id: &Uuid,
    ) -> Result<Option<Session>, sqlx::Error> {
        let session = sqlx::query_as!(
            Session,
            "SELECT * FROM sessions WHERE session_id = $1",
            session_id,
        )
        .fetch_optional(self.db_conn.get_pool())
        .await?;

        Ok(session)
    }

    pub async fn remove_expired_sessions(&self) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            Session,
            "DELETE FROM sessions WHERE expires_at < $1",
            Utc::now(),
        )
        .execute(self.db_conn.get_pool())
        .await?;

        Ok(())
    }
}
