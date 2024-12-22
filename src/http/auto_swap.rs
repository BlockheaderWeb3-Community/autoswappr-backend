use std::str::FromStr;

use super::types::{
    AutoSwapRequest, AutoSwapResponse, SubscriptionData, SwapSubscriptionFromTokenData,
};
use crate::api_error::ApiError;
use crate::config::env_var;
use crate::AppState;
use axum::{extract::State, Json};
// use serde_json::{json, Value};

use starknet::{
    accounts::{Account, ExecutionEncoding, SingleOwnerAccount},
    core::{
        chain_id,
        types::{Call, Felt},
        utils::get_selector_from_name,
    },
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Url,
    },
    signers::{LocalWallet, SigningKey},
};

async fn call_contract(from: &str, to: &str, value: f64) {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&env_var("STARKNET_RPC_URL")).unwrap(),
    ));

    let private_key =
        Felt::from_hex(env_var("PRIVATE_KEY").trim()).expect("Invalid PRIVATE_KEY format");

    // Initialize the wallet
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(private_key));
    // Initialize the account
    // println!("Account address: {}", private_key);

    let id: Felt = match env_var("APP_ENVIRONMENT").as_str() {
        "production" => chain_id::MAINNET,
        _ => chain_id::SEPOLIA,
    };

    let account_address = Felt::from_str(&env_var("OWNER_ADDRESS"));

    // println!("Account address: {}", account_address);

    let account = SingleOwnerAccount::new(
        provider,
        signer,
        account_address.unwrap(),
        id,
        ExecutionEncoding::New,
    );

    let calldata = vec![
        Felt::from_hex(from).unwrap(),
        Felt::from_hex(to).unwrap(),
        Felt::from(value as i128),
    ];

    let contract_address = Felt::from_hex(env_var("AUTOSWAP_CONTRACT_ADDRESS").as_str()).unwrap();

    let call = Call {
        to: contract_address,
        selector: get_selector_from_name("auto_swap").unwrap(),
        calldata,
    };

    match account.execute_v3(vec![call]).send().await {
        Ok(response) => {
            println!(
                "Call successful! Transaction hash: {}",
                response.transaction_hash
            );
        }
        Err(e) => {
            eprintln!("Error during call: {}", e);
        }
    }
}

pub async fn auto_swap(
    State(app_state): State<AppState>,
    Json(payload): Json<AutoSwapRequest>,
) -> Result<Json<AutoSwapResponse>, ApiError> {
    // validate_address(&payload.from)?;
    // validate_address(&payload.to);
    if !payload.from.starts_with("0x") && payload.from.len() != 42 {
        return Err(ApiError::InvalidRequest(
            "Invalid 'from' address format".to_string(),
        ));
    }
    if !payload.to.starts_with("0x") && payload.to.len() != 42 {
        return Err(ApiError::InvalidRequest(
            "Invalid 'to' address format".to_string(),
        ));
    }

    // println!("Payload: {:#?}", payload);

    let row: SubscriptionData = sqlx::query_as::<_, SubscriptionData>(
        r#"
        SELECT 
            wallet_address,
            to_token,
            is_active,
        FROM swap_subscription
        WHERE wallet_address = $1
        "#,
    )
    .bind(&payload.to)
    .fetch_one(&app_state.db.pool)
    .await
    .map_err(ApiError::DatabaseError)?;

    if !row.is_active {
        return Err(ApiError::InvalidRequest(
            "Subscription is not active".to_string(),
        ));
    }

    let row2: SwapSubscriptionFromTokenData = sqlx::query_as::<_, SwapSubscriptionFromTokenData>(
        r#"
        SELECT 
            wallet_address,
            from_token,
            percentage
        FROM swap_subscription
        WHERE wallet_address = $1
        "#,
    )
    .bind(row.wallet_address)
    .fetch_one(&app_state.db.pool)
    .await
    .map_err(ApiError::DatabaseError)?;

    call_contract(
        &row2.from_token,
        &payload.to,
        (row2.percentage as f64 / 100.0) * payload.value as f64,
    )
    .await;

    Ok(Json(AutoSwapResponse {
        status: "success".to_string(),
        message: "Auto swap started".to_string(),
    }))
}
