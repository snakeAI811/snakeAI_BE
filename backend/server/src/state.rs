use chrono::{Duration, Utc};
use database::{AppService, DatabasePool};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use twitter_v2::{authorization::Oauth2Client, oauth2::PkceCodeVerifier};
use utils::env::Env;

pub struct TwitterChallenge {
    pub verifier: PkceCodeVerifier,
    pub exp: i64,
}

impl TwitterChallenge {
    pub fn new(verifier: PkceCodeVerifier) -> Self {
        let now = Utc::now();
        let exp = now
            .checked_add_signed(Duration::seconds(2 * 60))
            .unwrap()
            .timestamp();
        Self { verifier, exp }
    }
}
pub struct OAuth2Ctx {
    pub client: Oauth2Client,
    pub challenges: HashMap<String, TwitterChallenge>,
}

impl OAuth2Ctx {
    pub fn init(env: &Env) -> Self {
        Self {
            client: Oauth2Client::new(
                &env.twitter_oauth_client_id,
                &env.twitter_oauth_client_secret,
                env.twitter_oauth_callback_url.clone(),
            ),
            challenges: HashMap::default(),
        }
    }

    pub fn remove_expired_challenges(&mut self) {
        let now = Utc::now().timestamp();
        self.challenges.retain(|_, challenge| challenge.exp < now);
    }
}

#[derive(Clone)]
pub struct AppState {
    pub env: Env,
    pub service: AppService,
    pub ctx: Arc<Mutex<OAuth2Ctx>>,
}

impl AppState {
    pub fn init(db: &Arc<DatabasePool>, env: Env) -> Self {
        Self {
            service: AppService::init(db, &env),
            ctx: Arc::new(Mutex::new(OAuth2Ctx::init(&env))),
            env,
        }
    }
}
