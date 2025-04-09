mod auth;
mod user;

use crate::state::AppState;
use axum::{http::HeaderValue, routing::get, Router};
use database::DatabasePool;
use hyper::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use utils::env::Env;

pub fn routes(db_conn: Arc<DatabasePool>, env: Env) -> Router {
    let production = env.production;
    let merged_router = {
        let app_state = AppState::init(&db_conn, env);
        Router::new()
            .merge(user::routes())
            .merge(auth::routes())
            .with_state(app_state)
            .merge(Router::new().route("/health", get(|| async { "<h1>SNAKE AI BACKEND</h1>" })))
            .merge(Router::new().route("/version", get(|| async { "V0.0.1" })))
    };

    let cors = CorsLayer::new()
        .allow_origin(if production {
            vec![]
        } else {
            vec!["http://localhost:3000".parse::<HeaderValue>().unwrap()]
        })
        .allow_methods([
            Method::POST,
            Method::GET,
            Method::PATCH,
            Method::DELETE,
            Method::PUT,
            Method::HEAD,
            Method::OPTIONS,
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app_router = Router::new().nest("/api/v1", merged_router).layer(cors);

    app_router
}
