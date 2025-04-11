use sqlx::types::Uuid;
use types::model::Reward;

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

    pub async fn get_rewards(
        &self,
        user_id: &Option<Uuid>,
        offset: Option<i64>,
        limit: Option<i64>,
        available: Option<bool>,
    ) -> Result<Vec<Reward>, sqlx::Error> {
        let offset = offset.unwrap_or_default();
        let limit = limit.unwrap_or(10);

        let mut filters = vec![];
        let mut index = 3;
        if user_id.is_some() {
            filters.push(format!("user_id = ${index}"));
            index += 1;
        }
        if available.is_some() {
            filters.push(format!("available = ${index}"));
        }

        let mut query = "SELECT * FROM rewards".to_string();

        if !filters.is_empty() {
            query.push_str(&format!(" WHERE {}", filters.join(" AND ")));
        }

        query.push_str(" ORDER BY created_at DESC OFFSET $1 LIMIT $2");

        let mut sql_query = sqlx::query_as::<_, Reward>(&query).bind(offset).bind(limit);

        if let Some(user_id) = user_id {
            sql_query = sql_query.bind(user_id);
        }

        if let Some(available) = available {
            sql_query = sql_query.bind(available);
        }

        let rewards = sql_query.fetch_all(self.db_conn.get_pool()).await?;

        Ok(rewards)
    }
}
