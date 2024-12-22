use axum::Router;

pub mod api_error;
pub mod config;
pub mod db;
pub mod http;
pub mod middleware;
pub mod service;
pub mod telemetry;
pub mod utils;

pub use config::*;
pub use db::*;

// App State to be shared accross requests.
#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub config: Config,
}

// Requests Router.
pub fn router(config: Config, db: Db) -> Router {
    // Initialize App State.
    let app_state = AppState { db, config };

    // Initialize Middlewares.
    let trace_layer = telemetry::trace_layer();
    let request_id_layer = middleware::request_id_layer();
    let propagate_request_id_layer = middleware::propagate_request_id_layer();
    let cors_layer = middleware::cors_layer();
    let timeout_layer = middleware::timeout_layer();
    let normalize_path_layer = middleware::normalize_path_layer();

    // Initialize and return Router.
    let router = http::router();
    Router::new()
        .merge(router)
        .layer(normalize_path_layer)
        .layer(cors_layer)
        .layer(timeout_layer)
        .layer(propagate_request_id_layer)
        .layer(trace_layer)
        .layer(request_id_layer)
        .with_state(app_state)
}
