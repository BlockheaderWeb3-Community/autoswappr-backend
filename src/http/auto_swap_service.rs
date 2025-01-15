use std::sync::Arc;

use super::types::{AutoSwapRequest, AutoSwapResponse, PoolKey, SwapData, SwapParameters, I129};
use crate::AppState;
use axum::{extract::State, http::StatusCode, Json};
use starknet::accounts::{Account, ExecutionEncoding, SingleOwnerAccount};
use starknet::core::codec::Encode;
use starknet::signers::{LocalWallet, SigningKey};
use starknet::{
    core::{
        chain_id,
        types::{BlockId, BlockTag, Call, Felt, U256},
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
        let wallet_address = payload.to.clone();
        let from_token = preference.from_token;
        let percentage = preference.percentage;
        let swap_amount: u128 = (payload.value * percentage as i64 / 100)
            .try_into()
            .unwrap();

        let rpc_url = std::env::var("RPC_URL").unwrap();
        let provider = create_rpc_provider(rpc_url.as_str()).unwrap();

        let private_key = std::env::var("PRIVATE_KEY").unwrap();

        let signer = LocalWallet::from(SigningKey::from_secret_scalar(
            Felt::from_hex(&private_key).unwrap(),
        ));
        let address = Felt::from_hex(&wallet_address).unwrap();
        let mut account = SingleOwnerAccount::new(
            provider.clone(),
            signer,
            address,
            chain_id::MAINNET,
            ExecutionEncoding::New,
        );

        let contract_address =
            Felt::from_hex("0x06657fa0b7490cea7fe27e7f955c6fff14e457d37dfa763d264a1b214d350065")
                .unwrap();
        let token0 = Felt::from_hex(&from_token).unwrap();
        let token1 = Felt::from_hex(&to_token).unwrap();
        let tick_spacing = (1000) as u128;

        let pool_key = PoolKey {
            token0,
            token1,
            fee: 170141183460469235273462165868118016,
            tick_spacing,
            extension: Felt::ZERO,
        };

        let swap_parameters = SwapParameters {
            amount: I129 {
                mag: swap_amount,
                sign: false,
            },
            is_token1: false,
            sqrt_ratio_limit: U256::from(18446748437148339061u128), // min sqrt ratio limit
            skip_ahead: 0,
        };

        let swap_data = SwapData {
            params: swap_parameters,
            pool_key,
            caller: address,
        };

        account.set_block_id(BlockId::Tag(BlockTag::Pending));

        let mut serialized = vec![];
        swap_data.encode(&mut serialized).unwrap();

        let transfer_call = Call {
            to: token0,
            selector: selector!("transfer"),
            calldata: vec![contract_address, Felt::from(swap_amount), Felt::ZERO],
        };

        let swap_call = Call {
            to: contract_address,
            selector: selector!("swap"),
            calldata: serialized,
        };

        let execution_result = account
            .execute_v3(vec![transfer_call, swap_call])
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
