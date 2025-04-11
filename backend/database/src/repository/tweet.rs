use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};
use types::model::Tweet;

use crate::pool::DatabasePool;
use std::sync::Arc;

#[derive(Clone)]
pub struct TweetRepository {
    db_conn: Arc<DatabasePool>,
}

impl TweetRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn insert_tweet(
        &self,
        user_id: &Uuid,
        tweet_id: &str,
        created_at: &DateTime<Utc>,
    ) -> Result<Tweet, sqlx::Error> {
        let tweet = sqlx::query_as!(
            Tweet,
            r#"
            INSERT INTO tweets (user_id, tweet_id, created_at)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            user_id,
            tweet_id,
            created_at
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(tweet)
    }
}
