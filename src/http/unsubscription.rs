use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{api_error::ApiError, AppState};

#[derive(Debug, Deserialize)]
pub struct UnsubscriptionPayload {
    pub wallet_address: String,
    pub from_token: String,
}

pub async fn handle_unsubscribe(
    State(state): State<AppState>,
    Json(payload): Json<UnsubscriptionPayload>,
) -> Result<Json<Value>, ApiError> {
    // Validate wallet_address format
    if !payload.wallet_address.starts_with("0x") || payload.wallet_address.len() != 42 {
        return Err(ApiError::InvalidRequest(
            "Invalid wallet address format".to_string(),
        ));
    }

    // Validate from_token format
    if !payload.from_token.starts_with("0x") || payload.from_token.len() != 42 {
        return Err(ApiError::InvalidRequest(
            "Invalid token address format".to_string(),
        ));
    }

    sqlx::query!(
        r#"
        DELETE FROM swap_subscription_from_token
        WHERE wallet_address = $1 AND from_token = $2
        "#,
        payload.wallet_address,
        payload.from_token
    )
    .execute(&state.db.pool)
    .await
    .map_err(ApiError::DatabaseError)?;

    Ok(Json(json!({
        "status": "success",
        "message": "Successfully unsubscribed"
    })))
}
