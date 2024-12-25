use std::sync::Arc;

use super::types::{
    AutoSwapRequest, AutoSwapResponse, PoolKey, RouteNode, Swap, TokenAmount, I129,
};
use crate::AppState;
use axum::{extract::State, http::StatusCode, Json};
use starknet::accounts::{Account, ExecutionEncoding, SingleOwnerAccount};
use starknet::core::codec::Encode;
use starknet::signers::{LocalWallet, SigningKey};
use starknet::{
    core::{
        chain_id,
        types::{Call, Felt, U256},
    },
    macros::selector,
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Url,
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
        let swap_amount: u128 = (payload.value * percentage as i64 / 100)
            .try_into()
            .unwrap();

        let rpc_url = std::env::var("RPC_URL").unwrap();
        let provider = create_rpc_provider(rpc_url.as_str()).unwrap();

        let private_key = std::env::var("PRIVATE_KEY").unwrap();
        let address = std::env::var("ADDRESS").unwrap();

        let signer = LocalWallet::from(SigningKey::from_secret_scalar(
            Felt::from_hex(&private_key).unwrap(),
        ));
        let address = Felt::from_hex(&address).unwrap();
        let mut account = SingleOwnerAccount::new(
            provider.clone(),
            signer,
            address,
            chain_id::MAINNET,
            ExecutionEncoding::New,
        );

        let contract_address =
            Felt::from_hex("0x0199741822c2dc722f6f605204f35e56dbc23bceed54818168c4c49e4fb8737e")
                .unwrap();
        let token0 = Felt::from_hex(from_token).unwrap();
        let token1 = Felt::from_hex(to_token).unwrap();
        let tick_spacing = (0.02 * 10000.0) as u128;

        let pool_key = PoolKey {
            token0,
            token1,
            fee: calculate_fee(0.01),
            tick_spacing,
            extension: Felt::ZERO,
        };

        let route_node = RouteNode {
            pool_key,
            sqrt_ratio_limit: U256::from(u128::MAX),
            skip_ahead: 0,
        };

        let token_amount = TokenAmount {
            token: token0,
            amount: I129 {
                mag: swap_amount,
                sign: false,
            },
        };

        let swap: Swap = Swap {
            route: route_node,
            token_amount,
        };

        let mut serialized = vec![];
        swap.encode(&mut serialized).unwrap();

        let execution_result = account
            .execute_v1(vec![Call {
                to: contract_address,
                selector: selector!("swap"),
                calldata: serialized,
            }])
            .send()
            .await;

        match execution_result {
            Ok(_) => Ok(Json(AutoSwapResponse {
                message: format!(
                    "Successfully swapped {} {} to {}",
                    swap_amount, from_token, to_token
                ),
            })),
            Err(e) => {
                eprintln!("Swap call failed: {:?}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        Ok(Json(AutoSwapResponse {
            message: "No swap preferences found for this wallet address".to_string(),
        }))
    }
}
