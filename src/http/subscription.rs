use axum::{extract::Query, extract::State, http::StatusCode, Json};

use super::types::{
    is_valid_address, CreateSubscriptionRequest, GetSubscriptionRequest, GetSubscriptionResponse,
    SubscriptionData, SuccessResponse,
};
use crate::api_error::ApiError;
use crate::AppState;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use serde_json::{json, Value};

const LIMIT: i32 = 10;

pub async fn create_subscription(
    State(state): State<AppState>,
    Json(payload): Json<CreateSubscriptionRequest>,
) -> Result<Json<SuccessResponse>, StatusCode> {
    let CreateSubscriptionRequest {
        wallet_address,
        to_token,
        from_token,
        percentage,
    } = payload;

    if from_token.len() != percentage.len() {
        return Err(StatusCode::BAD_REQUEST);
    }

    for token in &from_token {
        match is_valid_address(token) {
            true => continue,
            false => return Err(StatusCode::BAD_REQUEST),
        }
    }

    if !is_valid_address(&to_token) {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !is_valid_address(&wallet_address) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut tx = state
        .db
        .pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query!(
        r#"
        INSERT INTO swap_subscription (wallet_address, to_token, is_active)
        VALUES ($1, $2, true)
        ON CONFLICT (wallet_address)
        DO UPDATE SET to_token = $2, is_active = true, updated_at = NOW()
        "#,
        wallet_address,
        to_token,
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for (token, percentage) in from_token.iter().zip(percentage.iter()) {
        sqlx::query!(
            r#"
            INSERT INTO swap_subscription_from_token
            (wallet_address, from_token, percentage)
            VALUES ($1, $2, $3)
            "#,
            wallet_address,
            token,
            percentage,
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SuccessResponse { success: true }))
}

pub async fn get_subscription(
    State(state): State<AppState>,
    Query(params): Query<GetSubscriptionRequest>,
) -> Result<Json<Value>, ApiError> {
    let cursor: String = match params.cursor {
        Some(cursor1) => match OffsetDateTime::parse(&cursor1, &Rfc3339) {
            Ok(cur) => cur.format(&Rfc3339).unwrap(),
            Err(_) => return Err(ApiError::InvalidRequest("Invalid cursor".to_string())),
        },

        None => {
            let now = OffsetDateTime::now_utc();
            now.format(&Rfc3339).unwrap()
        }
    };

    let rows: Vec<SubscriptionData> = sqlx::query_as::<_, SubscriptionData>(
    r#"
        SELECT
            swap_subscription_from_token.from_token AS from_token,
            swap_subscription.to_token AS to_token,
            swap_subscription_from_token.percentage AS percentage,
            swap_subscription.is_active AS is_active,
            TO_CHAR(swap_subscription_from_token.created_at, 'YYYY-MM-DD"T"HH24:MI:SSZ') AS created_at
        FROM swap_subscription_from_token
        INNER JOIN swap_subscription ON swap_subscription_from_token.wallet_address = swap_subscription.wallet_address
        WHERE swap_subscription_from_token.created_at < $1::TIMESTAMPTZ
        AND swap_subscription_from_token.wallet_address = $2;
        "#
    )
        .bind(cursor)
        .bind(&params.wallet_address)
        .fetch_all(&state.db.pool)
        .await
        .map_err(ApiError::DatabaseError)?;

    let mut response_data: GetSubscriptionResponse = GetSubscriptionResponse {
        data: rows
            .into_iter()
            .map(|row| SubscriptionData {
                from_token: row.from_token,
                to_token: row.to_token,
                percentage: row.percentage,
                is_active: row.is_active,
                created_at: row.created_at,
            })
            .collect(),
        next_cursor: None,
    };

    match response_data.data.len() == LIMIT as usize {
        true => {
            response_data.next_cursor = Some(response_data.data.last().unwrap().created_at.clone());
        }
        false => {
            response_data.next_cursor = None;
        }
    };

    Ok(Json(json!(response_data)))
}
