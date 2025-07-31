mod handler;
mod middleware;
mod routes;
mod services;
mod state;
pub mod utils2;

use database::DatabasePool;
use routes::routes;
use std::{net::SocketAddr, sync::Arc};
use utils::env;

pub async fn run() {
    let env = env::Env::init();
    let connection = DatabasePool::init(&env)
        .await
        .unwrap_or_else(|e| panic!("Database error: {e}"));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env.port))
        .await
        .unwrap();
    println!("✅ Server running on localhost:{}", env.port);
    axum::serve(
        listener,
        routes(Arc::new(connection), env).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap_or_else(|e| panic!("Server error: {e}"));
}
