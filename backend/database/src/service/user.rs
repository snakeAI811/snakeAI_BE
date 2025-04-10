use sqlx::types::{chrono::Utc, Uuid};
use std::sync::Arc;
use types::{
    error::{ApiError, DbError},
    model::{Reward, Session, User},
};

use crate::{pool::DatabasePool, repository::UserRepository};

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            user_repo: UserRepository::new(db_conn),
        }
    }

    pub async fn insert_user(
        &self,
        twitter_id: &str,
        twitter_username: &str,
    ) -> Result<User, ApiError> {
        self.user_repo
            .insert_user(twitter_id, twitter_username)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_user_by_wallet_address(
        &self,
        wallet_address: &str,
    ) -> Result<Option<User>, ApiError> {
        self.user_repo
            .get_user_by_wallet_address(wallet_address)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn set_wallet_address(
        &self,
        user_id: &Uuid,
        wallet_address: &str,
    ) -> Result<User, ApiError> {
        self.user_repo
            .set_wallet_address(user_id, wallet_address)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn create_session(
        &self,
        user_id: &Uuid,
        user_agent: &str,
        ip_address: &str,
        session_ttl_in_minutes: u64,
    ) -> Result<Session, ApiError> {
        self.user_repo
            .create_session(user_id, user_agent, ip_address, session_ttl_in_minutes)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_user_by_session_id(
        &self,
        session_id: &Uuid,
        user_agent: &str,
    ) -> Result<User, ApiError> {
        let session = self
            .user_repo
            .get_session_by_session_id(session_id)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()))?;

        let now = Utc::now();
        if session.expires_at < now {
            return Err(ApiError::SessionExpired);
        }

        if session.user_agent != user_agent {
            return Err(ApiError::SessionInvalid);
        }

        self.user_repo
            .get_user_by_user_id(&session.user_id)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_rewards(
        &self,
        user_id: &Option<Uuid>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<Reward>, ApiError> {
        self.user_repo
            .get_rewards(user_id, offset, limit)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }
}
