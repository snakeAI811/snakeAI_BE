use crate::pool::DatabasePool;
use std::sync::Arc;

#[derive(Clone)]
pub struct ValuesRepository {
    db_conn: Arc<DatabasePool>,
}

impl ValuesRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn set_value(&self, key: &str, value: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO values (key, value) VALUES ($1, $2) ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value",
            key,
            value
        )
        .execute(self.db_conn.get_pool())
        .await?;

        Ok(())
    }

    pub async fn get_value(&self, key: &str) -> Result<Option<String>, sqlx::Error> {
        let result = sqlx::query_scalar!(
            "SELECT value FROM values WHERE key = $1",
            key
        )
        .fetch_optional(self.db_conn.get_pool())
        .await?;

        Ok(result)
    }

    pub async fn delete_value(&self, key: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM values WHERE key = $1",
            key
        )
        .execute(self.db_conn.get_pool())
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_all_values(&self) -> Result<Vec<(String, String)>, sqlx::Error> {
        let results = sqlx::query!(
            "SELECT key, value FROM values ORDER BY key"
        )
        .fetch_all(self.db_conn.get_pool())
        .await?;

        Ok(results.into_iter().map(|row| (row.key, row.value)).collect())
    }
}