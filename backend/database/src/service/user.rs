use sqlx::types::Uuid;
use std::sync::Arc;
use types::{
    error::{ApiError, DbError},
    model::{Session, User},
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
}
