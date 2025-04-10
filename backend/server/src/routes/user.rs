use crate::{
    handler::user::{get_me, set_wallet_address},
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
}
