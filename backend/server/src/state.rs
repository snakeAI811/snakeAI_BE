use database::{AppService, DatabasePool};
use std::sync::Arc;
use utils::env::Env;

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
