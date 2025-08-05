use sqlx::types::{
    Uuid,
    chrono::{DateTime, Utc},
};
use types::model::{Reward, RewardToReply, RewardWithUserAndTweet};

use crate::pool::DatabasePool;
use std::sync::Arc;

#[derive(Clone)]
pub struct RewardRepository {
    db_conn: Arc<DatabasePool>,
}

impl RewardRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn insert_reward(
        &self,
        user_id: &Uuid,
        tweet_id: &Uuid,
    ) -> Result<Reward, sqlx::Error> {
        let reward = sqlx::query_as::<_, Reward>(
            r#"
            INSERT INTO rewards (user_id, tweet_id, phase, reward_amount)
            VALUES ($1, $2, 'phase1', 100)
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(tweet_id)
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(reward)
    }

    pub async fn insert_reward_with_phase_and_amounts(
        &self,
        user_id: &Uuid,
        tweet_id: &Uuid,
        phase: i32,
        reward_amount: u64,
        burn_amount: u64,
    ) -> Result<Reward, sqlx::Error> {
        let phase_str = format!("phase{}", phase);
        let reward = sqlx::query_as::<_, Reward>(
            r#"
            INSERT INTO rewards (user_id, tweet_id, phase, reward_amount)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(tweet_id)
        .bind(&phase_str)
        .bind(reward_amount as i64)
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(reward)
    }

    pub async fn get_phase2_mining_count(
        &self,
        user_id: &Uuid,
    ) -> Result<i64, sqlx::Error> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM rewards WHERE user_id = $1 AND phase = 'phase2'"
        )
        .bind(user_id)
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(count)
    }

    pub async fn has_mined_in_phase2(
        &self,
        user_id: &Uuid,
    ) -> Result<bool, sqlx::Error> {
        let count = self.get_phase2_mining_count(user_id).await?;
        Ok(count > 0)
    }

    pub async fn get_available_reward(
        &self,
        user_id: &Uuid,
    ) -> Result<Option<Reward>, sqlx::Error> {
        let reward = sqlx::query_as!(
            Reward,
            "SELECT * FROM rewards WHERE user_id = $1 AND available = true",
            user_id,
        )
        .fetch_optional(self.db_conn.get_pool())
        .await?;

        Ok(reward)
    }

    pub async fn get_reward_by_id(&self, reward_id: &Uuid) -> Result<Option<Reward>, sqlx::Error> {
        let reward = sqlx::query_as::<_, Reward>("SELECT * FROM rewards WHERE id = $1")
            .bind(reward_id)
            .fetch_optional(self.db_conn.get_pool())
            .await?;

        Ok(reward)
    }

    pub async fn get_rewards(
        &self,
        user_id: &Option<Uuid>,
        offset: Option<i64>,
        limit: Option<i64>,
        available: Option<bool>,
    ) -> Result<Vec<RewardWithUserAndTweet>, sqlx::Error> {
        let offset = offset.unwrap_or_default();
        let limit = limit.unwrap_or(10);

        let mut filters = vec![];
        let mut index = 3;
        if user_id.is_some() {
            filters.push(format!("rewards.user_id = ${index}"));
            index += 1;
        }
        if available.is_some() {
            filters.push(format!("rewards.available = ${index}"));
        }

        let mut query = r#"
        SELECT
            rewards.*, users.twitter_id, users.twitter_username, tweets.tweet_id tweet_twitter_id
        FROM rewards
        JOIN users ON users.id = rewards.user_id
        JOIN tweets ON tweets.id = rewards.tweet_id"#   
            .to_string();

        if !filters.is_empty() {
            query.push_str(&format!(" WHERE {}", filters.join(" AND ")));
        }

        query.push_str(" ORDER BY rewards.created_at DESC OFFSET $1 LIMIT $2");

        let mut sql_query = sqlx::query_as::<_, RewardWithUserAndTweet>(&query)
            .bind(offset)
            .bind(limit);

        if let Some(user_id) = user_id {
            sql_query = sql_query.bind(user_id);
        }

        if let Some(available) = available {
            sql_query = sql_query.bind(available);
        }

        let rewards = sql_query.fetch_all(self.db_conn.get_pool()).await?;

        Ok(rewards)
    }

    pub async fn get_reward_balance(&self, user_id: &Uuid) -> Result<i64, sqlx::Error> {
        let total_balance = sqlx::query_scalar!(
            "SELECT CAST(SUM(reward_amount) AS BIGINT) FROM rewards WHERE user_id = $1 AND available = false",
            user_id
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(total_balance.unwrap_or_default())
    }

    pub async fn get_phase1_mining_total(&self, user_id: &Uuid) -> Result<i64, sqlx::Error> {
        let total: Option<i64> = sqlx::query_scalar(
            "SELECT CAST(SUM(reward_amount) AS BIGINT) FROM rewards WHERE user_id = $1 AND (phase = 'phase1' OR phase IS NULL)"
        )
        .bind(user_id)
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(total.unwrap_or_default())
    }

    pub async fn get_rewards_to_send_message(&self) -> Result<Vec<RewardToReply>, sqlx::Error> {
        let rewards = sqlx::query_as!(
            RewardToReply,
            "SELECT rewards.id, tweets.tweet_id, rewards.media_id, rewards.media_id_expires_at FROM rewards JOIN tweets ON rewards.tweet_id = tweets.id WHERE rewards.available = true AND rewards.message_sent = false",
        )
        .fetch_all(self.db_conn.get_pool())
        .await?;

        Ok(rewards)
    }

    pub async fn mark_as_message_sent(&self, reward_id: &Uuid) -> Result<Reward, sqlx::Error> {
        let reward = sqlx::query_as::<_, Reward>(
            "UPDATE rewards SET message_sent = true WHERE id = $1 RETURNING *"
        )
        .bind(reward_id)
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(reward)
    }

    pub async fn update_reward(
        &self,
        reward_id: &Uuid,
        transaction_signature: &str,
        reward_amount: i64,
        wallet_address: &str,
        block_time: &DateTime<Utc>,
        available: bool,
    ) -> Result<Reward, sqlx::Error> {
        let reward = sqlx::query_as::<_, Reward>(
            r#"
            UPDATE rewards SET transaction_signature = $2, reward_amount = $3, wallet_address = $4, block_time = $5, available = $6
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(reward_id)
        .bind(transaction_signature)
        .bind(reward_amount)
        .bind(wallet_address)
        .bind(block_time)
        .bind(available)
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(reward)
    }

    pub async fn update_reward_media_data(
        &self,
        reward_id: &Uuid,
        media_id: &str,
        media_id_expires_at: &DateTime<Utc>,
    ) -> Result<Reward, sqlx::Error> {
        let reward = sqlx::query_as::<_, Reward>(
            r#"
            UPDATE rewards SET media_id = $2, media_id_expires_at = $3
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(reward_id)
        .bind(media_id)
        .bind(media_id_expires_at)
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(reward)
    }

    pub async fn set_reward_flag(
        &self,
        user_id: &Uuid,
        tweet_id: &str,
    ) -> Result<bool, sqlx::Error> {
        // I have to get uuid from tweet_id in tweets table
        let tweet = sqlx::query!(
            "SELECT id FROM tweets WHERE tweet_id = $1 AND user_id = $2",
            tweet_id,
            user_id
        )
        .fetch_optional(self.db_conn.get_pool())
        .await?;    
        if tweet.is_none() {
            return Ok(false);
        } 
        let tweet_uuid = tweet.unwrap().id;
        let result = sqlx::query!(
            "UPDATE rewards SET available = false WHERE tweet_id = $1 AND user_id = $2 RETURNING id",
            tweet_uuid,
            user_id
        )
        .fetch_optional(self.db_conn.get_pool())
        .await?;

        Ok(result.is_some())
    }

    pub async fn get_rewards_count(&self) -> Result<i64, sqlx::Error> {
        let reward = sqlx::query_scalar!("SELECT COUNT(*) FROM rewards")
            .fetch_one(self.db_conn.get_pool())
            .await?;

        Ok(reward.unwrap_or_default())
    }
}
