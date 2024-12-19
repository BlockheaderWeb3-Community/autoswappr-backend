use autoswappr_backend::{telemetry, Configuration, Db};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Read (development) Environment Variables.
    dotenvy::dotenv().ok();

    // Setup telemetry.
    telemetry::setup_tracing();

    // App configuration.
    tracing::debug!("Initializing configuration");
    let config = Configuration::new();

    // Initialize DB connection.
    tracing::debug!("Initializing DB pool");
    let db = Db::new(&config.db_str, config.db_pool_max_size)
        .await
        .expect("Failed to initialize DB");

    // Run migrations.
    tracing::debug!("Running Migrations");
    db.migrate().await.expect("Failed to run migrations");

    // Listen for requests on specified port.
    tracing::info!("Starting server on {}", config.listen_address);
    let listener = TcpListener::bind(&config.listen_address)
        .await
        .expect("Failed to bind address");

    // Spin up router.
    let router = autoswappr_backend::router(config, db);

    // Serve requests.
    axum::serve(listener, router)
        .await
        .expect("Failed to start server")
}
