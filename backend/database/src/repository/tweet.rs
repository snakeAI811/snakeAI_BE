use crate::pool::DatabasePool;
use sqlx::types::{
    Uuid,
    chrono::{DateTime, Utc},
};
use std::sync::Arc;
use types::model::{Tweet, TweetWithUser};

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
        mining_phase: &str,
        reward_amount: u64,
    ) -> Result<Tweet, sqlx::Error> {
        let phase_number = if mining_phase == "Phase2" { 2 } else { 1 };
        
        let tweet = sqlx::query_as!(
            Tweet,
            r#"
            INSERT INTO tweets (user_id, tweet_id, created_at, mining_phase, reward_amount)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, tweet_id, created_at, mining_phase, false as rewarded, reward_amount
            "#,
            user_id,
            tweet_id,
            created_at,
            phase_number,
            reward_amount as i64
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(tweet)
    }

    pub async fn get_tweets_count(&self) -> Result<i64, sqlx::Error> {
        let tweet = sqlx::query_scalar!("SELECT COUNT(*) FROM tweets")
            .fetch_one(self.db_conn.get_pool())
            .await?;

        Ok(tweet.unwrap_or_default())
    }

    pub async fn get_tweets_count_by_userid(&self, user_id:Uuid) -> Result<i64, sqlx::Error> {
        let tweet = sqlx::query_scalar!("SELECT count(*)
            FROM tweets 
            INNER JOIN rewards ON tweets.id = rewards.tweet_id 
            WHERE tweets.user_id = $1",
            user_id)
            .fetch_one(self.db_conn.get_pool())
            .await?;

        Ok(tweet.unwrap_or_default())
    }

    pub async fn get_tweets(
        &self,
        user_id: &Option<Uuid>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<TweetWithUser>, sqlx::Error> {
        let offset = offset.unwrap_or_default();
        let limit = limit.unwrap_or(10);

        let base_query = "
            SELECT 
                tweets.id, tweets.tweet_id, tweets.created_at, tweets.mining_phase, 
                users.twitter_id, users.twitter_username, 
                CONCAT('Tweet #', tweets.tweet_id) AS content, 
                tweets.rewarded, tweets.reward_amount 
            FROM tweets 
            JOIN users ON tweets.user_id = users.id
            JOIN rewards ON tweets.id = rewards.tweet_id
        ";

        let mut query = String::from(base_query);
        let sql_query;

        if let Some(user_id) = user_id {
            query.push_str(" WHERE tweets.user_id = $1 ORDER BY tweets.created_at DESC LIMIT $2 OFFSET $3");
            sql_query = sqlx::query_as::<_, TweetWithUser>(&query)
                .bind(user_id)
                .bind(limit)
                .bind(offset);
        } else {
            query.push_str(" ORDER BY tweets.created_at DESC LIMIT $1 OFFSET $2");
            sql_query = sqlx::query_as::<_, TweetWithUser>(&query)
                .bind(limit)
                .bind(offset);
        }

        let tweets = sql_query.fetch_all(self.db_conn.get_pool()).await?;
        Ok(tweets)
    }



    pub async fn get_phase2_mining_count(&self, user_id: &Uuid) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM tweets WHERE user_id = $1 AND mining_phase = 2",
            user_id
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(count.unwrap_or_default())
    }

    pub async fn get_phase1_mining_count(&self, user_id: &Uuid) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM tweets WHERE user_id = $1 AND mining_phase = 1",
            user_id
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(count.unwrap_or_default())
    }

    pub async fn get_all_phase2_mining_count(&self) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM tweets WHERE mining_phase = 2"
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(count.unwrap_or_default())
    }

    pub async fn get_all_phase1_mining_count(&self) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM tweets WHERE mining_phase = 1"
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(count.unwrap_or_default())
    }

    pub async fn get_tweets_by_phase(
        &self,
        user_id: &Option<String>,
        mining_phase: &str,
    ) -> Result<Vec<Tweet>, sqlx::Error> {
        let phase_number = if mining_phase == "Phase2" { 2 } else { 1 };
        
        match user_id {
            Some(uid) => {
                let parsed_id = Uuid::parse_str(uid)
                    .map_err(|_| sqlx::Error::RowNotFound)?;
                
                let tweets = sqlx::query_as!(
                    Tweet,
                    r#"
                    SELECT id, user_id, tweet_id, created_at, mining_phase, false as rewarded, 0::bigint as reward_amount
                    FROM tweets 
                    WHERE user_id = $1 AND mining_phase = $2
                    ORDER BY created_at DESC
                    "#,
                    parsed_id,
                    phase_number
                )
                .fetch_all(self.db_conn.get_pool())
                .await?;

                Ok(tweets)
            }
            None => {
                let tweets = sqlx::query_as!(
                    Tweet,
                    r#"
                    SELECT id, user_id, tweet_id, created_at, mining_phase, false as rewarded, 0::bigint as reward_amount
                    FROM tweets 
                    WHERE mining_phase = $1
                    ORDER BY created_at DESC
                    "#,
                    phase_number
                )
                .fetch_all(self.db_conn.get_pool())
                .await?;

                Ok(tweets)
            }
        }
    }
}
