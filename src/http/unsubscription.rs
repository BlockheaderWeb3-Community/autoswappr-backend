use axum::{extract::State, Json};
use serde::Deserialize;

use super::types::{is_valid_address, SuccessResponse};
use crate::{api_error::ApiError, AppState};

#[derive(Debug, Deserialize)]
pub struct UnsubscriptionPayload {
    pub wallet_address: String,
    pub from_token: String,
}

pub async fn handle_unsubscribe(
    State(state): State<AppState>,
    Json(payload): Json<UnsubscriptionPayload>,
) -> Result<Json<SuccessResponse>, ApiError> {
    let UnsubscriptionPayload {
        wallet_address,
        from_token,
    } = payload;

    // Validate wallet_address format
    if !is_valid_address(&wallet_address) {
        return Err(ApiError::InvalidRequest(
            "Invalid wallet address format".to_string(),
        ));
    }

    // Validate from_token format
    if !is_valid_address(&from_token) {
        return Err(ApiError::InvalidRequest(
            "Invalid token address format".to_string(),
        ));
    }

    sqlx::query!(
        r#"
        DELETE FROM swap_subscription_from_token
        WHERE wallet_address = $1 AND from_token = $2
        "#,
        wallet_address,
        from_token
    )
    .execute(&state.db.pool)
    .await
    .map_err(ApiError::DatabaseError)?;

    Ok(Json(SuccessResponse { success: true }))
}
