use std::{sync::Arc, time::Duration};

use sqlx::types::{chrono::Utc, Uuid};
use types::model::{Session, User};

use crate::pool::DatabasePool;

#[derive(Clone)]
pub struct UserRepository {
    db_conn: Arc<DatabasePool>,
}

impl UserRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn insert_user(
        &self,
        twitter_id: &str,
        twitter_username: &str,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (twitter_id, twitter_username)
            VALUES ($1, $2)
            ON CONFLICT (twitter_id) DO UPDATE
            SET twitter_username = EXCLUDED.twitter_username
            RETURNING *
            "#,
            twitter_id,
            twitter_username,
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(user)
    }

    pub async fn set_wallet_address(
        &self,
        user_id: &Uuid,
        wallet_address: &str,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users SET wallet_address = $1 WHERE id = $2
            RETURNING *
            "#,
            wallet_address,
            user_id,
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_user_id(&self, user_id: &Uuid) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users WHERE id = $1
            "#,
            user_id,
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_wallet_address(
        &self,
        wallet_address: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users WHERE wallet_address = $1
            "#,
            wallet_address,
        )
        .fetch_optional(self.db_conn.get_pool())
        .await?;

        Ok(user)
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
    ) -> Result<Session, sqlx::Error> {
        let session = sqlx::query_as!(
            Session,
            r#"
                SELECT * FROM sessions WHERE session_id = $1
            "#,
            session_id,
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(session)
    }

    pub async fn remove_expired_sessions(&self) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            Session,
            r#"
                DELETE FROM sessions WHERE expires_at < $1
            "#,
            Utc::now(),
        )
        .execute(self.db_conn.get_pool())
        .await?;

        Ok(())
    }
}
