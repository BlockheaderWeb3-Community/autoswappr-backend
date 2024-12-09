use crate::AppState;
use crate::api_error::ApiError;

use axum::{extract::State, http::StatusCode, Json};

use serde_json::{json, Value};

use super::types::{AutoSwapRequest, AutoSwapResponse, SubscriptionData, SwapSubscriptionFromTokenData};
use super::utils::{validate_address, call_contract};
// use crate::api_error::ApiError;


pub async fn auto_swap_q(
    State(app_state): State<AppState>,
    Json(payload): Json<AutoSwapRequest>,
) -> Result<Json<AutoSwapResponse>, StatusCode> {
    
    println!("RRRR: {:#?}", app_state.config.env);

    println!("Payload: {:#?}", payload);

    // function body here
    Ok(Json(AutoSwapResponse {
        status: "success".to_string(),
        message: "Auto swap started".to_string(),
    }))
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

    // call_contract().await;

    // if !payload.from.starts_with("0x") && payload.from.len() != 42 {
    //     return Err(ApiError::InvalidRequest(
    //         "Invalid token address format".to_string(),
    //     ));
    // }

    println!("RRRR: {:#?}", app_state.config.env);

    println!("Payload: {:#?}", payload);
    

    // let row: Subscription_data = sqlx::query_as::<_, Subscription_data>(
    //     r#"
    //     SELECT 
    //         wallet_address,
    //         to_token,
    //         is_active,
    //     FROM swap_subscription
    //     WHERE wallet_address = $1
    //     "#,
    // )
    // .bind(payload.to)
    // .fetch_one(&app_state.db.pool)
    // .await
    // .map_err(ApiError::DatabaseError)?;

    // if !row.is_active {
    //     return Err(ApiError::InvalidRequest(
    //         "Subscription is not active".to_string(),
    //     ));
    // }


    // let row2: Swap_subscription_from_token_data = sqlx::query_as::<_, Swap_subscription_from_token_data>(
    //     r#"
    //     SELECT 
    //         wallet_address,
    //         from_token,
    //         percentage
    //     FROM swap_subscription
    //     WHERE wallet_address = $1
    //     "#,
    // )
    // .bind(row.wallet_address)
    // .fetch_one(&app_state.db.pool)
    // .await
    // .map_err(ApiError::DatabaseError)?;

    // call_contract().await;

    // Ok(Json(json!({"status":"success"})))
    Ok(Json(AutoSwapResponse {
        status: "success".to_string(),
        message: "Auto swap started".to_string(),
    }))
}