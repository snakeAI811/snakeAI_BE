use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair},
    Client, Cluster, Program,
};
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
        self.challenges.retain(|_, challenge| challenge.exp > now);
    }
}

#[derive(Clone)]
pub struct AppState {
    pub env: Env,
    pub service: AppService,
    pub ctx: Arc<Mutex<OAuth2Ctx>>,
    pub program: Arc<Program<Arc<Keypair>>>,
}

impl AppState {
    pub fn init(db: &Arc<DatabasePool>, env: Env) -> Self {
        let payer = Arc::new(Keypair::from_base58_string(&env.backend_wallet_private_key));
        let client =
            Client::new_with_options(Cluster::Devnet, payer, CommitmentConfig::confirmed());
        let program = Arc::new(client.program(snake_contract::ID).unwrap());
        Self {
            service: AppService::init(db, &env),
            ctx: Arc::new(Mutex::new(OAuth2Ctx::init(&env))),
            env,
            program,
        }
    }
}
