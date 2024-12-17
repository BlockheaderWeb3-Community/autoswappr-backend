use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde_json::{json, Value};

use super::types::{ActivityLogData, ActivityLogGetRequest, ActivityLogGetResponse};
use crate::api_error::ApiError;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

const LIMIT: i32 = 10;

pub async fn log_retrieval(
    State(app_state): State<AppState>,
    Query(query_params): Query<ActivityLogGetRequest>,
) -> Result<Json<Value>, ApiError> {
    // Add default date if no cursor is provided
    let cursor: String = match query_params.cursor {
        Some(cursor1) => match OffsetDateTime::parse(&cursor1, &Rfc3339) {
            Ok(cur) => cur.format(&Rfc3339).unwrap(),
            Err(_) => return Err(ApiError::InvalidRequest("Invalid cursor".to_string())),
        },

        None => {
            let now = OffsetDateTime::now_utc();
            now.format(&Rfc3339).unwrap()
        }
    };
    let initial_query = r#"
        SELECT
            wallet_address,
            from_token,
            to_token,
            amount_from,
            percentage,
            amount_to,
            TO_CHAR(created_at, 'YYYY-MM-DD"T"HH24:MI:SSZ') AS created_at
        FROM transactions_log
        WHERE created_at < $1::TIMESTAMPTZ
    "#;
    let mut conditions = vec![];
    if let Some(wallet_address) = query_params.wallet_address {
        conditions.push(format!("AND wallet_address = '{}'", wallet_address));
    }

    if let Some(from_token) = query_params.from_token {
        conditions.push(format!("AND from_token = '{}'", from_token));
    }

    if let Some(to_token) = query_params.to_token {
        conditions.push(format!("AND to_token = '{}'", to_token));
    }

    if let Some(amount_to) = query_params.amount_to {
        conditions.push(format!("AND amount_to = '{}'", amount_to));
    }

    let query_build = format!(
        r#"
        {}
        {}
        ORDER BY created_at DESC
        LIMIT $2
        "#,
        initial_query,
        conditions.join(" ")
    );
    let rows: Vec<ActivityLogData> = sqlx::query_as::<_, ActivityLogData>(&query_build)
        .bind(cursor)
        .bind(LIMIT)
        .fetch_all(&app_state.db.pool)
        .await
        .map_err(ApiError::DatabaseError)?;

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
    match response_data.transactions.len() == LIMIT as usize {
        true => {
            response_data.next_cursor = Some(
                response_data
                    .transactions
                    .last()
                    .unwrap()
                    .created_at
                    .clone(),
            );
        }
        false => {
            response_data.next_cursor = None;
        }
    };

    Ok(Json(json!(response_data)))
}
