use crate::pool::DatabasePool;
use std::sync::Arc;
use types::model::Value;

#[derive(Clone)]
pub struct UtilRepository {
    db_conn: Arc<DatabasePool>,
}

impl UtilRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn get_latest_tweet_id(&self) -> Result<Option<String>, sqlx::Error> {
        let value = sqlx::query_as!(Value, "SELECT * from values WHERE key = 'latest_tweet_id'",)
            .fetch_optional(self.db_conn.get_pool())
            .await?;

        Ok(value.map(|v| v.value))
    }

    pub async fn upsert_latest_tweet_id(
        &self,
        latest_tweet_id: &str,
    ) -> Result<String, sqlx::Error> {
        let value = sqlx::query_as!(
            Value,
            r#"
                INSERT INTO values (key, value)
                VALUES ('latest_tweet_id', $1)
                ON CONFLICT (key)
                DO UPDATE SET value = EXCLUDED.value
                RETURNING *
            "#,
            latest_tweet_id,
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(value.value)
    }
}
