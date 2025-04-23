use crate::{
    handler::user::{
        get_claim_tx, get_me, get_profile, get_rewards, get_tweets, set_wallet_address,
        token_validation,
    },
    state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/token", get(token_validation))
        .route("/me", get(get_me))
        .route("/profile", get(get_profile))
        .route("/wallet_address", post(set_wallet_address))
        .route("/rewards", get(get_rewards))
        .route("/tweets", get(get_tweets))
        .route("/claim_tx", post(get_claim_tx))
}
