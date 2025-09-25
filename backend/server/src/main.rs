#[tokio::main]
async fn main() {
    // Initialize logger, default level = info if not set
    env_logger::init();

    server::run().await
}
