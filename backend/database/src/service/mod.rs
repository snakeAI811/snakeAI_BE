mod reward;
mod session;
mod tweet;
mod user;
mod util;
mod otc_swap;

pub use reward::*;
pub use session::*;
pub use tweet::*;
pub use user::*;
pub use util::*;
pub use otc_swap::*;

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
    pub otc_swap: OtcSwapService,
}

impl AppService {
    pub fn init(db: &Arc<DatabasePool>, _env: &Env) -> Self {
        Self {
            reward: RewardService::new(db),
            session: SessionService::new(db),
            tweet: TweetService::new(db),
            user: UserService::new(db),
            util: UtilService::new(db),
            otc_swap: OtcSwapService::new(db.clone()),
        }
    }
}
