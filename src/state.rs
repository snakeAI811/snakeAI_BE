use crate::database::{AppService, DatabasePool};
use crate::env::Env;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub env: Env,
    pub service: AppService,
}

impl AppState {
    pub fn init(db: &Arc<DatabasePool>, env: Env) -> Self {
        Self {
            service: AppService::init(db, &env),
            env,
        }
    }
}
