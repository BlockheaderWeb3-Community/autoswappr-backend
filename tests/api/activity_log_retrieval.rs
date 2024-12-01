use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde::{Deserialize, Serialize};

use crate::helpers::*;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ActivityLogGetResponse {
    pub transactions: Vec<ActivityLogData>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ActivityLogData {
    pub wallet_address: String,
    pub from_token: String,
    pub to_token: String,
    pub percentage: i16,
    pub amount_from: i64,
    pub amount_to: i64,
    pub created_at: String,
}

async fn populate_db(pool: &PgPool) -> bool {
    println!("----------------populating db");
    sqlx::query(
        "INSERT INTO transactions_log (
            wallet_address,
            from_token,
            to_token,
            percentage,
            amount_from,
            amount_to,
            created_at,
            updated_at
        ) VALUES
            ('0x1234567890abcdef8234567890abcdef12345678', '0x9876543210fedcba9876543210fedcba98765432', '0x1111111111111111111111111111111111111111', 70, 1870000000, 500600000, '2024-11-29 10:49:42.728841+00', NULL),
            ('0x1234567890abcdef8234567890abcdef12345678', '0x9876543210fedcba9876543210fedcba98765432', '0x1111111111111111111111111111111111111111', 70, 1870000000, 500600000, '2024-11-29 10:49:42.316783+00', NULL),
            ('0x1234567890abcdef8234567890abcdef12345678', '0x9876543210fedcba9876543210fedcba98765432', '0x1111111111111111111111111111111111111111', 70, 1870000000, 500600000, '2024-11-29 10:49:41.917281+00', NULL),
            ('0x1234567890abcdef8234567890abcdef12345678', '0x9876543210fedcba9876543210fedcba98765432', '0x1111111111111111111111111111111111111111', 70, 1870000000, 500600000, '2024-11-29 10:49:41.514413+00', NULL),
            ('0x1234567890abcdef8234567890abcdef12345678', '0x9876543210fedcba9876543210fedcba98765432', '0x1111111111111111111111111111111111111111', 70, 1870000000, 500600000, '2024-11-29 10:49:41.08329+00', NULL),
            ('0x1234567890abcdef8234567890abcdef12345678', '0x9876543210fedcba9876543210fedcba98765432', '0x1111111111111111111111111111111111111111', 70, 1870000000, 500600000, '2024-11-29 10:49:40.562681+00', NULL),
            ('0x1234567890abcdef8234567890abcdef12345678', '0x9876543210fedcba9876543210fedcba98765432', '0x1111111111111111111111111111111111111111', 70, 1870000000, 500600000, '2024-11-29 10:49:40.053961+00', NULL),
            ('0x1234567890abcdef8234567890abcdef12345678', '0x9876543210fedcba9876543210fedcba98765432', '0x1111111111111111111111111111111111111111', 70, 1870000000, 500600000, '2024-11-29 10:49:39.507289+00', NULL),
            ('0x1234567890abcdef8234567890abcdef12345678', '0x9876543210fedcba9876543210fedcba98765432', '0x1111111111111111111111111111111111111111', 70, 1870000000, 500600000, '2024-11-29 10:49:38.464406+00', NULL),
            ('0x1234567890abcdef8234567890abcdef12345678', '0x9876543210fedcba9876543210fedcba98765432', '0x1111111111111111111111111111111111111111', 70, 1870000000, 500600000, '2024-11-29 10:49:36.202316+00', NULL),
            ('0x1234567890abcdef1234567890abcdef12345678', '0x9876543210fedcba9876543210fedcba98765432', '0x1111111111111111111111111111111111111111', 50, 100000000, 50000000, '2024-11-28 12:02:49.898622+00', NULL),
            ('0x1234567890abcdef1234567890abcdef12345678', '0x9876543210fedcba9876543210fedcba98765432', '0x1111111111111111111111111111111111111111', 50, 100000000, 50000000, '2024-11-28 12:02:47.453754+00', NULL),
            ('0x1234567890abcdef1234567890abcdef12345678', '0x9876543210fedcba9876543210fedcba98765432', '0x1111111111111111111111111111111111111111', 50, 100000000, 50000000, '2024-11-28 12:02:42.457038+00', NULL);

            ",).execute(pool).await.unwrap();
    true
}

#[tokio::test]
async fn test_log_retrieval_pagination() {
    let app = TestApp::new().await;

    sqlx::query!("DELETE FROM transactions_log")
        .execute(&app.db.pool)
        .await
        .unwrap();

    let req = Request::get("/log_retrieval?cursor=2024-11-30T10:49:36.20Z&limit=10")
        .body(Body::empty())
        .unwrap();
    let resp = app.request(req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body_bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let response_body: ActivityLogGetResponse = serde_json::from_slice(&body_bytes).unwrap();
    // println!("1: ///////////////////{:#?}", response_body);

    assert_eq!(
        response_body.transactions.len(),
        0,
        "Expected no transactions"
    );

    let _t = populate_db(&app.db.pool).await;

    let req = Request::get("/log_retrieval?cursor=2024-11-30T10:49:36Z&limit=10")
        .body(Body::empty())
        .unwrap();
    let resp = app.request(req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body_bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let response_body: ActivityLogGetResponse = serde_json::from_slice(&body_bytes).unwrap();
    // println!("2: ///////////////////{:#?}", response_body);

    assert_eq!(response_body.transactions.len(), 10);

    let next_cursor = response_body.next_cursor.unwrap();

    // println!("Next Cursor: {}", next_cursor);

    assert_eq!(next_cursor, "2024-11-29T10:49:36Z".to_string());
    let url = format!("/log_retrieval?cursor={}&limit=10", next_cursor);

    let req = Request::get(&url).body(Body::empty()).unwrap();

    let resp = app.request(req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body_bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let response_body: ActivityLogGetResponse = serde_json::from_slice(&body_bytes).unwrap();
    // println!("3:///////////////////{:#?}", response_body);

    assert_eq!(response_body.transactions.len(), 3);

    assert_eq!(response_body.next_cursor, None);
}

#[tokio::test]
async fn test_log_retrieval_no_cursor() {
    let app = TestApp::new().await;

    let req = Request::get("/log_retrieval?limit=10")
        .body(Body::empty())
        .unwrap();
    let resp = app.request(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_log_retrieval_no_cursor_no_limit() {
    let app = TestApp::new().await;

    let req = Request::get("/log_retrieval").body(Body::empty()).unwrap();
    let resp = app.request(req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_log_retrieval_invalid_cursor() {
    let app = TestApp::new().await;

    let req = Request::get("/log_retrieval?cursor=invalid")
        .body(Body::empty())
        .unwrap();
    let resp = app.request(req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_log_retrieval_invalid_limit() {
    let app = TestApp::new().await;

    let req = Request::get("/log_retrieval?limit=invalid")
        .body(Body::empty())
        .unwrap();
    let resp = app.request(req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
