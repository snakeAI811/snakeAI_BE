mod user;

pub use user::*;

use crate::DatabasePool;
use std::sync::Arc;
use utils::env::Env;

#[derive(Clone)]
pub struct AppService {
    pub user: UserService,
}

impl AppService {
    pub fn init(db: &Arc<DatabasePool>, _env: &Env) -> Self {
        Self {
            user: UserService::new(db),
        }
    }
}
