use std::sync::Arc;
use types::error::{ApiError, DbError};

use crate::{pool::DatabasePool, repository::UtilRepository};

#[derive(Clone)]
pub struct UtilService {
    util_repo: UtilRepository,
}

impl UtilService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            util_repo: UtilRepository::new(db_conn),
        }
    }

    pub async fn get_latest_tweet_id(&self) -> Result<Option<String>, ApiError> {
        self.util_repo
            .get_latest_tweet_id()
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn upsert_latest_tweet_id(&self, latest_tweet_id: &str) -> Result<String, ApiError> {
        self.util_repo
            .upsert_latest_tweet_id(latest_tweet_id)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }
}
