use crate::{
    handler::auth::{callback, check_reward_available, login},
    state::AppState,
};
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(login))
        .route("/callback", get(callback))
        .route("/check_reward_available", get(check_reward_available))
}
