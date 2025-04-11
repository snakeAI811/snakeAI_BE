use sqlx::types::Uuid;
use types::{
    error::{ApiError, DbError},
    model::Reward,
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
}
