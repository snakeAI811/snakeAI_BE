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
}
