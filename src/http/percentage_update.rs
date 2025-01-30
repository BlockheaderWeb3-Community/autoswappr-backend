use super::types::{is_valid_address, SuccessResponse, UpdatePercentageRequest};
use crate::AppState;
use axum::{extract::State, http::StatusCode, Json};

pub async fn update_percentage(
    State(state): State<AppState>,
    Json(payload): Json<UpdatePercentageRequest>,
) -> Result<Json<SuccessResponse>, StatusCode> {
    let UpdatePercentageRequest {
        wallet_address,
        from_token,
        percentage,
    } = payload;

    // wallet address validation
    if !is_valid_address(&wallet_address) {
        return Err(StatusCode::BAD_REQUEST);
    }
    if percentage <= 0 || percentage > 100 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // percent updte
    let result = sqlx::query!(
        r#"
        UPDATE swap_subscription_from_token
        SET percentage = $1, updated_at = NOW()
        WHERE wallet_address = $2 AND from_token = $3
        "#,
        percentage,
        wallet_address,
        from_token
    )
    .execute(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(SuccessResponse { success: true }))
}
