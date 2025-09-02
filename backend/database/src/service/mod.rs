mod reward;
mod session;
mod tweet;
mod tweet_template;
mod user;
mod util;
mod otc_swap;
mod values;

pub use reward::*;
pub use session::*;
pub use tweet::*;
pub use tweet_template::*;
pub use user::*;
pub use util::*;
pub use otc_swap::*;
pub use values::*;

use crate::DatabasePool;
use crate::ValuesRepository;
use std::sync::Arc;
use utils::env::Env;
use sqlx::types::Uuid;
use types::model::RewardWithUserAndTweet;

#[derive(Clone)]
pub struct AppService {
    pub reward: RewardService,
    pub session: SessionService,
    pub tweet: TweetService,
    pub tweet_template: TweetTemplateService,
    pub user: UserService,
    pub util: UtilService,
    pub otc_swap: OtcSwapService,
    pub values: ValuesService,
}

impl AppService {
    pub fn init(db: &Arc<DatabasePool>, _env: &Env) -> Self {
        Self {
            reward: RewardService::new(db),
            session: SessionService::new(db),
            tweet: TweetService::new(db),
            tweet_template: TweetTemplateService::new(db),
            user: UserService::new(db),
            util: UtilService::new(db),
            otc_swap: OtcSwapService::new(db.clone()),
            values: ValuesService::new(ValuesRepository::new(db)),
        }
    }

    /// Get user rewards - delegates to reward service
    pub async fn get_user_rewards(
        &self,
        user_id: &Uuid,
        limit: Option<i64>,
    ) -> Result<Vec<RewardWithUserAndTweet>, types::error::ApiError> {
        self.reward.get_rewards(&Some(*user_id), None, limit, None).await
    }
}
