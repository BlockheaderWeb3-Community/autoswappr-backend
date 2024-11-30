use axum::{
    routing::{get, post},
    Router,
};
mod activity_log_retrieval;
mod health_check;
mod subscription;
mod types;
mod unsubscription;
use crate::AppState;

// Application router.
// All routes should be merged here.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health_check", get(health_check::health_check))
        .route("/unsubscribe", post(unsubscription::handle_unsubscribe))
        .route("/subscriptions", post(subscription::create_subscription))
        .route("/log_retrieval", get(activity_log_retrieval::log_retrieval))
}
