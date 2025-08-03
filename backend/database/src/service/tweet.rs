use crate::{pool::DatabasePool, repository::TweetRepository};
use sqlx::types::{
    Uuid,
    chrono::{DateTime, Utc},
};
use std::sync::Arc;
use types::{
    error::{ApiError, DbError},
    model::{Tweet, TweetWithUser},
};

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
        mining_phase: &str,
        reward_amount: u64,
    ) -> Result<Tweet, ApiError> {
        self.tweet_repo
            .insert_tweet(user_id, tweet_id, created_at, mining_phase, reward_amount)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_tweets_count(&self) -> Result<i64, ApiError> {
        self.tweet_repo
            .get_tweets_count()
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_tweets(
        &self,
        user_id: &Option<Uuid>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<TweetWithUser>, ApiError> {
        self.tweet_repo
            .get_tweets(user_id, offset, limit)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_phase2_mining_count(&self, user_id: &Uuid) -> Result<i64, ApiError> {
        self.tweet_repo
            .get_phase2_mining_count(user_id)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_phase1_mining_count(&self, user_id: &Uuid) -> Result<i64, ApiError> {
        self.tweet_repo
            .get_phase1_mining_count(user_id)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_tweets_by_phase(
        &self,
        user_id: &Option<Uuid>,
        mining_phase: &str,
    ) -> Result<Vec<Tweet>, ApiError> {
        let user_id_str = user_id.as_ref().map(|id| id.to_string());
        self.tweet_repo
            .get_tweets_by_phase(&user_id_str, mining_phase)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }
}
