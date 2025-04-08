mod database;
mod env;
mod routes;
mod state;

use database::DatabasePool;
use routes::routes;
use std::sync::Arc;

pub async fn run() {
    let env = env::Env::init();
    let connection = DatabasePool::init(&env)
        .await
        .unwrap_or_else(|e| panic!("Database error: {e}"));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env.port))
        .await
        .unwrap();
    axum::serve(listener, routes(Arc::new(connection), env))
        .await
        .unwrap_or_else(|e| panic!("Server error: {e}"));
}
