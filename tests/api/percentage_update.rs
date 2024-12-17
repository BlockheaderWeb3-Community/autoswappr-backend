use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, Request, StatusCode},
};
use serde_json::json;
use sqlx::PgPool;

use crate::helpers::*;

async fn setup_test_data(
    pool: &PgPool,
    wallet_address: &str,
    from_token: &str,
    initial_percentage: i16,
) {
    sqlx::query!(
        r#"
        INSERT INTO swap_subscription (wallet_address, to_token, is_active)
        VALUES ($1, $2, true)
        ON CONFLICT (wallet_address)
        DO UPDATE SET to_token = $2, is_active = true, updated_at = NOW()
        "#,
        wallet_address,
        "0x1234567890123456789012345678901234567890"
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query!(
        r#"
            INSERT INTO swap_subscription_from_token
            (wallet_address, from_token, percentage)
            VALUES ($1, $2, $3)
            "#,
        wallet_address,
        from_token,
        initial_percentage,
    )
    .execute(pool)
    .await
    .unwrap();
}

#[tokio::test]
async fn test_update_percentage_success() {
    let app = TestApp::new().await;

    clean_database(&app.db.pool).await;

    let wallet_address = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
    let from_token = "0xabcdef0123456789abcdef0123456789abcdef01";
    let initial_percentage = 50;

    setup_test_data(&app.db.pool, wallet_address, from_token, initial_percentage).await;

    let payload = json!({
        "wallet_address": wallet_address,
        "from_token": from_token,
        "percentage": 75
    });

    let req = Request::builder()
        .method("POST")
        .uri("/update-percentage")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    let resp = app.request(req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let updated = sqlx::query!(
        r#"
        SELECT percentage 
        FROM swap_subscription_from_token
        WHERE wallet_address = $1 AND from_token = $2
        "#,
        wallet_address,
        from_token
    )
    .fetch_one(&app.db.pool)
    .await
    .unwrap();

    assert_eq!(updated.percentage, 75);
}

#[tokio::test]
async fn test_update_percentage_not_found() {
    let app = TestApp::new().await;

    clean_database(&app.db.pool).await;

    let payload = json!({
        "wallet_address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
        "from_token": "0xabcdef0123456789abcdef0123456789abcdef01",
        "percentage": 75
    });

    let req = Request::builder()
        .method("POST")
        .uri("/update-percentage")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    let resp = app.request(req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
