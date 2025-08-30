use crate::{pool::DatabasePool, repository::UserRepository, SessionRepository};
use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};
use std::sync::Arc;
use types::{
    error::{ApiError, DbError},
    model::User,
};

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    session_repo: SessionRepository,
}

impl UserService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            user_repo: UserRepository::new(db_conn),
            session_repo: SessionRepository::new(db_conn),
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

    pub async fn get_user_by_twitter_id(&self, twitter_id: &str) -> Result<Option<User>, ApiError> {
        self.user_repo
            .get_user_by_twitter_id(twitter_id)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_user_by_session_id(
        &self,
        session_id: &Uuid,
        user_agent: &str,
    ) -> Result<User, ApiError> {
        let session = self
            .session_repo
            .get_session_by_session_id(session_id)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()))?
            .ok_or(ApiError::SessionInvalid)?;

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

    pub async fn set_wallet_address(
        &self,
        user_id: &Uuid,
        wallet_address: &str,
    ) -> Result<User, ApiError> {
        self.user_repo
            .set_wallet_address_by_uuid(user_id, wallet_address)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn set_latest_claim_timestamp(
        &self,
        user_id: &Uuid,
        latest_claim_timestamp: &DateTime<Utc>,
    ) -> Result<User, ApiError> {
        self.user_repo
            .set_latest_claim_timestamp(user_id, latest_claim_timestamp)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>, ApiError> {
        self.user_repo
            .get_user_by_user_id(user_id)
            .await
            .map(Some)
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn update_patron_status(
        &self,
        user_id: &Uuid,
        patron_status: &str,
    ) -> Result<User, ApiError> {
        self.user_repo
            .update_patron_status(user_id, patron_status)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn update_role(
        &self,
        user_id: &Uuid,
        role: &str,
    ) -> Result<User, ApiError> {
        self.user_repo
            .update_role(user_id, role)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn update_lock_details(
        &self,
        user_id: &Uuid,
        lock_duration_months: i32,
        lock_amount: i64,
    ) -> Result<User, ApiError> {
        self.user_repo
            .update_lock_details(user_id, lock_duration_months, lock_amount)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn set_wallet_address_by_uuid(
        &self,
        user_id: &Uuid,
        wallet_address: &str,
    ) -> Result<User, ApiError> {
        self.user_repo
            .set_wallet_address_by_uuid(user_id, wallet_address)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn update_is_following_by_twitter_id(
        &self,
        twitter_id: &str,
        is_following: bool,
    ) -> Result<User, ApiError> {
        self.user_repo
            .update_is_following_by_twitter_id(twitter_id, is_following)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn add_accumulated_reward(
        &self,
        user_id: &Uuid,
        reward_amount: i64,
    ) -> Result<User, ApiError> {
        self.user_repo
            .add_accumulated_reward(user_id, reward_amount)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }
}
