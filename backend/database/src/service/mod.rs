mod tweet;
mod user;
mod util;

pub use tweet::*;
pub use user::*;
pub use util::*;

use crate::DatabasePool;
use std::sync::Arc;
use utils::env::Env;

#[derive(Clone)]
pub struct AppService {
    pub user: UserService,
    pub util: UtilService,
    pub tweet: TweetService,
}

impl AppService {
    pub fn init(db: &Arc<DatabasePool>, _env: &Env) -> Self {
        Self {
            user: UserService::new(db),
            util: UtilService::new(db),
            tweet: TweetService::new(db),
        }
    }
}
