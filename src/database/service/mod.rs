mod user;

pub use user::*;

use crate::{env::Env, DatabasePool};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppService {
    pub user: UserService,
}

impl AppService {
    pub fn init(db: &Arc<DatabasePool>, env: &Env) -> Self {
        Self {
            user: UserService::new(db),
        }
    }
}
