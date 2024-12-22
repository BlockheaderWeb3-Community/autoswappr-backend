use axum::{extract::State, Json, http::StatusCode};
use sqlx::PgPool;
use crate::{AppState, types::{AutoSwapRequest, AutoSwapResponse}};
use ethers::prelude::*;

pub async fn handle_auto_swap(
    State(state): State<AppState>,
    Json(payload): Json<AutoSwapRequest>,
) -> Result<Json<AutoSwapResponse>, StatusCode> {

    if payload.value <= 0 || !payload.to.starts_with("0x") {
        return Err(StatusCode::BAD_REQUEST);
    }

    let subscription = sqlx::query!(
        "SELECT to_token FROM swap_subscription WHERE wallet_address = $1",
        payload.to
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if subscription.is_none() {
        return Ok(Json(AutoSwapResponse {
            message: "No subscription found for this wallet address".to_string(),
        }));
    }

    let to_token = subscription.unwrap().to_token;

    let swap_preferences = sqlx::query!(
        "SELECT percentage FROM swap_subscription_from_token 
         WHERE wallet_address = $1 AND from_token = 'STRK'",
        payload.to
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(preference) = swap_preferences {
        let percentage = preference.percentage;
        let swap_amount = payload.value * percentage as i64 / 100;

        let result = execute_swap(state.ekubo_contract.clone(), to_token, swap_amount).await;
        if result.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        return Ok(Json(AutoSwapResponse {
            message: format!("Successfully swapped {} STRK to {}", swap_amount, to_token),
        }));
    }

    Ok(Json(AutoSwapResponse {
        message: "No swap preferences found for this wallet address".to_string(),
    }))
}

async fn execute_swap(
    contract: Address,
    to_token: String,
    amount: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO
    Ok(())
}
