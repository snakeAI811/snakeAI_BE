use sqlx::types::{
    Uuid,
    chrono::{DateTime, Utc},
};
use types::{
    error::{ApiError, DbError},
    model::{Reward, RewardToReply},
};

use crate::{pool::DatabasePool, repository::RewardRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct RewardService {
    reward_repo: RewardRepository,
}

impl RewardService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            reward_repo: RewardRepository::new(db_conn),
        }
    }

    pub async fn insert_reward(&self, user_id: &Uuid, tweet_id: &Uuid) -> Result<Reward, ApiError> {
        self.reward_repo
            .insert_reward(user_id, tweet_id)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_available_reward(&self, user_id: &Uuid) -> Result<Option<Reward>, ApiError> {
        self.reward_repo
            .get_available_reward(user_id)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_rewards(
        &self,
        user_id: &Option<Uuid>,
        offset: Option<i64>,
        limit: Option<i64>,
        available: Option<bool>,
    ) -> Result<Vec<Reward>, ApiError> {
        self.reward_repo
            .get_rewards(user_id, offset, limit, available)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_reward_balance(&self, user_id: &Uuid) -> Result<i64, ApiError> {
        self.reward_repo
            .get_reward_balance(user_id)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_rewards_to_send_message(&self) -> Result<Vec<RewardToReply>, ApiError> {
        self.reward_repo
            .get_rewards_to_send_message()
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn mark_as_message_sent(&self, reward_id: &Uuid) -> Result<Reward, ApiError> {
        self.reward_repo
            .mark_as_message_sent(reward_id)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn update_reward(
        &self,
        reward_id: &Uuid,
        transaction_signature: &str,
        reward_amount: i64,
        wallet_address: &str,
        block_time: &DateTime<Utc>,
        available: bool,
    ) -> Result<Reward, ApiError> {
        self.reward_repo
            .update_reward(
                reward_id,
                transaction_signature,
                reward_amount,
                wallet_address,
                block_time,
                available,
            )
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }
}
