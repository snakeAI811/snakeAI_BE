use crate::{handler::user::get_me, state::AppState};
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new().route("/me", get(get_me))
}
