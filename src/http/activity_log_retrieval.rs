use axum::Json;
use serde_json::{json, Value};
use axum::extract::{State, Query};

use crate::{api_error,AppState};

use crate::api_error::ApiError;

use super::types::{ActivityLogGetRequest, ActivityLogGetResponse, ActivityLogData};

pub async fn log_retrieval(
    State(app_state): State<AppState>,
    Query(query_params): Query<ActivityLogGetRequest>,
    ) -> Result<Json<Value>, ApiError> {

        println!("Log Retrieval: {:?}", query_params);
        
        let cursor = query_params.cursor.unwrap();
        let limit = query_params.limit.unwrap_or(10);

        
        let rows: Vec<(String, String, String, i64, i64, String)> = sqlx::query_as(
            r#"
            SELECT wallet_address, from_token, to_token, amount_from, amount_to, created_at
            FROM transactions_log
            WHERE created_at < $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
            .bind(cursor)
            .bind(limit)
            .fetch_all(&app_state.db.pool)  // Execute the query and fetch all results
            .await?;

        let data: Vec<ActivityLogData> = rows
            .into_iter()
            .map(|(wallet_address, from_token, to_token, amount_from, amount_to, created_at)| ActivityLogData {
                wallet_address,
                from_token,
                to_token,
                amount_from,
                amount_to,
                created_at,
            })
            .collect();

        let response_data: ActivityLogGetResponse = ActivityLogGetResponse {
            transactions: data
        };


        Ok(Json(json!(response_data)))

        

    }

