use chrono::{DateTime, Utc};
use dotenv;
use url::Url;

#[derive(Debug, Clone)]
pub struct Env {
    pub port: u32,
    pub session_ttl_in_minutes: u64,
    pub database_url: String,
    pub database_max_connections: u32,
    pub twitter_oauth_client_id: String,
    pub twitter_oauth_client_secret: String,
    pub twitter_oauth_callback_url: Url,
    pub twitter_bearer_token: String,
    pub twitter_api_key: String,
    pub twitter_api_key_secret: String,
    pub twitter_access_token: String,
    pub twitter_access_token_secret: String,
    pub twitter_job_schedule: String,
    pub frontend_url: String,
    pub production: bool,
    pub backend_wallet_private_key: String,
    pub token_mint: String,
    pub solana_job_schedule: String,
    // pub reward_sync_job_schedule: String,
    pub solana_rpc_url: String,
    pub solana_ws_url: String,
    pub play_snake_ai_id: String,
}

impl Env {
    pub fn init() -> Self {
        dotenv::dotenv().ok();
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8000);

        let session_ttl_in_minutes = std::env::var("SESSION_TTL_IN_MINUTES")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(24 * 60); // 1 day

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(10);

        let twitter_oauth_client_id =
            std::env::var("TWITTER_OAUTH_CLIENT_ID").expect("TWITTER_OAUTH_CLIENT_ID must be set");
        let twitter_oauth_client_secret = std::env::var("TWITTER_OAUTH_CLIENT_SECRET")
            .expect("TWITTER_OAUTH_CLIENT_SECRET must be set");
        let twitter_oauth_callback_url = std::env::var("TWITTER_OAUTH_CALLBACK_URL")
            .expect("TWITTER_OAUTH_CALLBACK_URL must be set")
            .parse()
            .expect("TWITTER_OAUTH_CALLBACK_URL is incorrect");
        let twitter_bearer_token =
            std::env::var("TWITTER_BEARER_TOKEN").expect("TWITTER_BEARER_TOKEN must be set");
        let twitter_api_key =
            std::env::var("TWITTER_API_KEY").expect("TWITTER_API_KEY must be set");
        let twitter_api_key_secret =
            std::env::var("TWITTER_API_KEY_SECRET").expect("TWITTER_API_KEY_SECRET must be set");
        let twitter_access_token =
            std::env::var("TWITTER_ACCESS_TOKEN").expect("TWITTER_ACCESS_TOKEN must be set");
        let twitter_access_token_secret = std::env::var("TWITTER_ACCESS_TOKEN_SECRET")
            .expect("TWITTER_ACCESS_TOKEN_SECRET must be set");

        let twitter_job_schedule =
            std::env::var("TWITTER_JOB_SCHEDULE").expect("TWITTER_JOB_SCHEDULE must be set");

        let frontend_url = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");

        let production = std::env::var("PRODUCTION")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or_default();

        let backend_wallet_private_key = std::env::var("BACKEND_WALLET_PRIVATE_KEY")
            .expect("BACKEND_WALLET_PRIVATE_KEY must be set");
        let token_mint = std::env::var("TOKEN_MINT").expect("TOKEN_MINT must be set");

        let solana_job_schedule =
            std::env::var("SOLANA_JOB_SCHEDULE").expect("SOLANA_JOB_SCHEDULE must be set");
        let solana_rpc_url = std::env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set");
        let solana_ws_url = std::env::var("SOLANA_WS_URL").unwrap_or_else(|_| solana_rpc_url.replace("http", "ws"));

        // let reward_sync_job_schedule =
        //     std::env::var("REWARD_SYNC_JOB_SCHEDULE").expect("REWARD_SYNC_JOB_SCHEDULE must be set");

        let play_snake_ai_id =
            std::env::var("PLAY_SNAKE_AI_ID").expect("PLAY_SNAKE_AI_ID must be set");

        Self {
            port,
            session_ttl_in_minutes,
            database_url,
            database_max_connections,
            twitter_oauth_client_id,
            twitter_oauth_client_secret,
            twitter_oauth_callback_url,
            twitter_bearer_token,
            twitter_api_key,
            twitter_api_key_secret,
            twitter_access_token,
            twitter_access_token_secret,
            twitter_job_schedule,
            frontend_url,
            production,
            backend_wallet_private_key,
            token_mint,
            solana_job_schedule,
            // reward_sync_job_schedule,
            solana_rpc_url,
            solana_ws_url,
            play_snake_ai_id,
        }
    }

    pub fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
