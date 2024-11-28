use axum::{routing::get, Router};
mod health_check;
mod types;
mod activity_log_retrieval;

use crate::AppState;

// Application router.
// All routes should be merged here.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health_check", get(health_check::health_check))
        .route("/log_retrieval", get(activity_log_retrieval::log_retrieval))

}