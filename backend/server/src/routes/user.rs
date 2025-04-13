use crate::{
    handler::user::{get_claim_tx, get_me, get_rewards, set_wallet_address},
    state::AppState,
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(get_me))
        .route("/wallet_address", post(set_wallet_address))
        .route("/rewards", get(get_rewards))
        .route("/claim_tx", post(get_claim_tx))
}
