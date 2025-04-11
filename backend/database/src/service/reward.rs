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
}
