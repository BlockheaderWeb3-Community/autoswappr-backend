use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::service::transaction_logs::log_transaction;
use crate::{api_error::ApiError, AppState};

#[derive(Debug, Deserialize)]
pub struct TransactionLogPayload {
    pub wallet_address: String,
    pub from_token: String,
    pub to_token: String,
    pub percentage: u16,
    pub amount_from: u64,
    pub amount_to: u64,
}

pub async fn log_transaction_to_db(
    State(state): State<AppState>,
    Json(payload): Json<TransactionLogPayload>,
) -> Result<Json<Value>, ApiError> {
    let tx = log_transaction(
        &payload.wallet_address,
        &payload.from_token,
        &payload.to_token,
        payload.percentage,
        payload.amount_from,
        payload.amount_to,
        &state.db.pool,
    )
    .await;
    match tx.is_ok() {
        true => Ok(Json(json!(
            {
                "status": "success",
                "message":"transaction logged successfully"
            }
        ))),
        false => Err(ApiError::InvalidRequest(tx.err().unwrap())),
    }
}
