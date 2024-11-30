use axum::{extract::State, http::StatusCode, Json};

use super::types::{CreateSubscriptionRequest, CreateSubscriptionResponse};
use crate::AppState;

pub async fn create_subscription(
    State(state): State<AppState>,
    Json(payload): Json<CreateSubscriptionRequest>,
) -> Result<Json<CreateSubscriptionResponse>, StatusCode> {
    if payload.from_token.len() != payload.percentage.len() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !payload.to_token.starts_with("0x") && payload.to_token.len() != 42 {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !payload.wallet_address.starts_with("0x") && payload.wallet_address.len() != 42 {
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
        payload.wallet_address,
        payload.to_token,
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for (token, percentage) in payload.from_token.iter().zip(payload.percentage.iter()) {
        sqlx::query!(
            r#"
            INSERT INTO swap_subscription_from_token
            (wallet_address, from_token, percentage)
            VALUES ($1, $2, $3)
            "#,
            payload.wallet_address,
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

    Ok(Json(CreateSubscriptionResponse {
        wallet_address: payload.wallet_address,
    }))
}
