use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde_json::{json, Value};

use crate::api_error::ApiError;

use super::types::{ActivityLogData, ActivityLogGetRequest, ActivityLogGetResponse};

pub async fn log_retrieval(
    State(app_state): State<AppState>,
    Query(query_params): Query<ActivityLogGetRequest>,
) -> Result<Json<Value>, ApiError> {
    // println!("\nLog Retrieval: {:?}\n", query_params);

    let cursor = query_params
        .cursor
        .ok_or_else(|| ApiError::InvalidRequest("Missing cursor".into()))?;
    let limit = query_params.limit.unwrap_or(10);

    let rows: Vec<ActivityLogData> = sqlx::query_as::<_, ActivityLogData>(
        r#"
        SELECT 
            wallet_address,
            from_token,
            to_token,
            amount_from,
            amount_to,
            percentage,
            TO_CHAR(created_at, 'YYYY-MM-DD"T"HH24:MI:SSZ') AS created_at 
        FROM transactions_log
        WHERE created_at < $1::TIMESTAMPTZ
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(cursor)
    .bind(limit)
    .fetch_all(&app_state.db.pool)
    .await
    .map_err(|err| ApiError::DatabaseError(err))?;

    // Map results to the response data structure
    let mut response_data: ActivityLogGetResponse = ActivityLogGetResponse {
        transactions: rows
            .into_iter()
            .map(|row| ActivityLogData {
                wallet_address: row.wallet_address,
                from_token: row.from_token,
                to_token: row.to_token,
                percentage: row.percentage,
                amount_from: row.amount_from,
                amount_to: row.amount_to,
                created_at: row.created_at,
            })
            .collect(),
        next_cursor: None,
    };

    // Check if there are more transactions
    if response_data.transactions.len() == limit as usize {
        let last_transaction = response_data.transactions.last().unwrap();
        response_data.next_cursor = Some(last_transaction.created_at.clone());
    }

    Ok(Json(json!(response_data)))
}
