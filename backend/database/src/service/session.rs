use crate::{pool::DatabasePool, SessionRepository};
use sqlx::types::Uuid;
use std::sync::Arc;
use types::{
    error::{ApiError, DbError},
    model::Session,
};

#[derive(Clone)]
pub struct SessionService {
    session_repo: SessionRepository,
}

impl SessionService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            session_repo: SessionRepository::new(db_conn),
        }
    }

    pub async fn create_session(
        &self,
        user_id: &Uuid,
        user_agent: &str,
        ip_address: &str,
        session_ttl_in_minutes: u64,
    ) -> Result<Session, ApiError> {
        self.session_repo
            .create_session(user_id, user_agent, ip_address, session_ttl_in_minutes)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }
}
