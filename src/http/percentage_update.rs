use super::types::{UpdatePercentageRequest, UpdatePercentageResponse};
use crate::AppState;
use axum::{extract::State, http::StatusCode, Json};

pub async fn update_percentage(
    State(state): State<AppState>,
    Json(payload): Json<UpdatePercentageRequest>,
) -> Result<Json<UpdatePercentageResponse>, StatusCode> {
    // wallet address validation
    if !payload.wallet_address.starts_with("0x") && payload.wallet_address.len() != 42 {
        return Err(StatusCode::BAD_REQUEST);
    }
    if payload.percentage <= 0 || payload.percentage >= 100 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // percent updte
    let result = sqlx::query!(
        r#"
        UPDATE swap_subscription_from_token 
        SET percentage = $1, updated_at = NOW()
        WHERE wallet_address = $2 AND from_token = $3
        "#,
        payload.percentage,
        payload.wallet_address,
        payload.from_token
    )
    .execute(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(UpdatePercentageResponse {
        message: "Percentage updated successfully".to_string(),
    }))
}
