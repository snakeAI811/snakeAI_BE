use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};
use std::sync::Arc;
use types::{
    error::{ApiError, DbError},
    model::Tweet,
};

use crate::{pool::DatabasePool, repository::TweetRepository};

#[derive(Clone)]
pub struct TweetService {
    tweet_repo: TweetRepository,
}

impl TweetService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            tweet_repo: TweetRepository::new(db_conn),
        }
    }

    pub async fn insert_tweet(
        &self,
        user_id: &Uuid,
        tweet_id: &str,
        created_at: &DateTime<Utc>,
    ) -> Result<Tweet, ApiError> {
        self.tweet_repo
            .insert_tweet(user_id, tweet_id, created_at)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }
}
