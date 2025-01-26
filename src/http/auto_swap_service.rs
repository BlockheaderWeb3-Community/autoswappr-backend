use super::types::{is_valid_address, AutoSwapRequest, SuccessResponse};
use crate::utils::ekubo::ekubo_swap;
use crate::AppState;
use axum::{extract::State, http::StatusCode, Json};
use starknet::core::types::Felt;

const DECIMALS: u128 = 1_000_000_000_000_000_000;

pub async fn handle_auto_swap(
    State(state): State<AppState>,
    Json(payload): Json<AutoSwapRequest>,
) -> Result<Json<SuccessResponse>, StatusCode> {
    let AutoSwapRequest {
        token_from,
        swap_recipient,
        value_received,
    } = payload;

    if value_received <= 0 || !is_valid_address(&token_from) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let swap_preferences = sqlx::query!(
        r#"
        SELECT s.to_token, sf.percentage
        FROM swap_subscription s
        INNER JOIN swap_subscription_from_token sf ON s.wallet_address = sf.wallet_address
        WHERE s.wallet_address = $1 AND sf.from_token = $2
        "#,
        swap_recipient,
        token_from
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let preference = match swap_preferences {
        Some(pref) => pref,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let swap_amount: u128 = (value_received * preference.percentage as i64 / 100)
        .try_into()
        .unwrap();
    let swap_amount = swap_amount * DECIMALS;

    let token0 = Felt::from_hex(&token_from).unwrap();
    let token1 = Felt::from_hex(&preference.to_token).unwrap();

    match ekubo_swap(token0, token1, swap_amount).await {
        Ok(_) => Ok(Json(SuccessResponse { success: true })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
