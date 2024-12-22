use std::sync::Arc;

use axum::{extract::State, Json, http::StatusCode};
use super::types::{AutoSwapRequest, AutoSwapResponse};
use crate::AppState;
use starknet::{
    core::types::{BlockId, Felt, BlockTag, FunctionCall},
    macros::{felt, selector},
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Url,
    },
};

fn create_rpc_provider(
    rpc_url: &str,
) -> Result<Arc<JsonRpcClient<HttpTransport>>, Box<dyn std::error::Error>> {
    let url = Url::parse(rpc_url)?;
    let provider = JsonRpcClient::new(HttpTransport::new(url));
    Ok(Arc::new(provider))
}

fn calculate_fee(fee_percentage: f64) -> u128 {
    let fee_decimal = fee_percentage / 100.0;
    let scale: f64 = 2.0f64.powi(128); // 2^128
    (fee_decimal * scale).floor() as u128
}

pub async fn handle_auto_swap(
    State(state): State<AppState>,
    Json(payload): Json<AutoSwapRequest>,
) -> Result<Json<AutoSwapResponse>, StatusCode> {

    dotenvy::dotenv().ok();

    if payload.value <= 0 || !payload.to.starts_with("0x") {
        return Err(StatusCode::BAD_REQUEST);
    }

    let subscription = sqlx::query!(
        r#"
        SELECT to_token
        FROM swap_subscription
        WHERE wallet_address = $1
        "#,
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
        r#"
        SELECT from_token, percentage
        FROM swap_subscription_from_token
        WHERE wallet_address = $1
        "#,
        payload.to
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(preference) = swap_preferences {
        let from_token = preference.from_token;
        let percentage = preference.percentage;
        let swap_amount = payload.value * percentage as i64 / 100;

        let rpc_url = std::env::var("RPC_URL").unwrap();
        let provider = create_rpc_provider(rpc_url.as_str()).unwrap();
        let contract_address =
            Felt::from_hex("0x00000005dd3D2F4429AF886cD1a3b08289DBcEa99A294197E9eB43b0e0325b4b")
                .unwrap();
        let token0 =
            Felt::from_hex(from_token)
                .unwrap();
        let token1 =
            Felt::from_hex(to_token)
                .unwrap();

        // TODO

        return Ok(Json(AutoSwapResponse {
            message: format!(
                "Successfully swapped {} {} to {}",
                swap_amount, from_token, to_token
            ),
        }));
    }


    Ok(Json(AutoSwapResponse {
        message: "No swap preferences found for this wallet address".to_string(),
    }))
}

