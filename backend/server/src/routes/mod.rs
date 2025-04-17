mod auth;
mod user;

use crate::{middleware::auth as auth_middleware, state::AppState};
use axum::{http::HeaderValue, middleware, routing::get, Router};
use database::DatabasePool;
use hyper::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, COOKIE},
    Method,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use utils::env::Env;

pub fn routes(db_conn: Arc<DatabasePool>, env: Env) -> Router {
    let _production = env.production;
    let merged_router = {
        let app_state = AppState::init(&db_conn, env);
        let protected =
            Router::new()
                .nest("/user", user::routes())
                .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    auth_middleware,
                )));
        let public = Router::new().merge(auth::routes());
        Router::new()
            .merge(protected)
            .merge(public)
            .with_state(app_state)
            .merge(Router::new().route("/health", get(|| async { "<h1>SNAKE AI BACKEND</h1>" })))
            .merge(Router::new().route("/version", get(|| async { "V0.0.1" })))
    };

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://170.130.55.155:3000".parse::<HeaderValue>().unwrap(),
            "https://snake-token.vercel.app"
                .parse::<HeaderValue>()
                .unwrap(),
        ])
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
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, COOKIE]);

    let app_router = Router::new().nest("/", merged_router).layer(cors);

    app_router
}
