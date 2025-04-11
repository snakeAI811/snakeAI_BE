mod reward;
mod session;
mod tweet;
mod user;
mod util;

pub use reward::*;
pub use session::*;
pub use tweet::*;
pub use user::*;
pub use util::*;

use crate::DatabasePool;
use std::sync::Arc;
use utils::env::Env;

#[derive(Clone)]
pub struct AppService {
    pub reward: RewardService,
    pub session: SessionService,
    pub tweet: TweetService,
    pub user: UserService,
    pub util: UtilService,
}

impl AppService {
    pub fn init(db: &Arc<DatabasePool>, _env: &Env) -> Self {
        Self {
            reward: RewardService::new(db),
            session: SessionService::new(db),
            tweet: TweetService::new(db),
            user: UserService::new(db),
            util: UtilService::new(db),
        }
    }
}
