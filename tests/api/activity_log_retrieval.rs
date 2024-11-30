use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde::{Deserialize, Serialize};

use crate::helpers::*;

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

#[tokio::test]
async fn test_log_retrieval() {
    let app = TestApp::new().await; // 2024-11-24T10:30:00Z
    let req = Request::get("/log_retrieval?cursor=2024-11-28T12:02:49Z&limit=10")
        .body(Body::empty())
        .unwrap();
    let resp = app.request(req).await;
    let headers = resp.headers().clone();

    assert_eq!(resp.status(), StatusCode::OK);
    assert!(headers.get("x-request-id").is_some());
    assert_eq!(headers.get("access-control-allow-origin").unwrap(), "*");
    assert!(headers.get("vary").is_some());

    let body_bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let response_body: ActivityLogGetResponse = serde_json::from_slice(&body_bytes).unwrap();
    // println!("///////////////////{:#?}", response_body);

    assert_eq!(
        response_body,
        ActivityLogGetResponse {
            transactions: vec![],
            next_cursor: None,
        }
    )
}

#[tokio::test]
async fn test_log_retrieval_no_cursor() {
    let app = TestApp::new().await;
    let req = Request::get("/log_retrieval?limit=10")
        .body(Body::empty())
        .unwrap();
    let resp = app.request(req).await;
    let headers = resp.headers().clone();
    assert_eq!(resp.status(), StatusCode::OK);
    assert!(headers.get("x-request-id").is_some());
    assert_eq!(headers.get("access-control-allow-origin").unwrap(), "*");
    assert!(headers.get("vary").is_some());

    let body_bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let response_body: ActivityLogGetResponse = serde_json::from_slice(&body_bytes).unwrap();
    // println!("///////////////////{:#?}", response_body);

    assert_eq!(
        response_body,
        ActivityLogGetResponse {
            transactions: vec![],
            next_cursor: None,
        }
    )
}

#[tokio::test]
async fn test_log_retrieval_no_cursor_no_limit() {
    let app = TestApp::new().await;

    let req = Request::get("/log_retrieval").body(Body::empty()).unwrap();
    let resp = app.request(req).await;
    let headers = resp.headers().clone();
    assert_eq!(resp.status(), StatusCode::OK);
    assert!(headers.get("x-request-id").is_some());
    assert_eq!(headers.get("access-control-allow-origin").unwrap(), "*");
    assert!(headers.get("vary").is_some());

    let body_bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let response_body: ActivityLogGetResponse = serde_json::from_slice(&body_bytes).unwrap();
    // println!("///////////////////{:#?}", response_body);

    assert_eq!(
        response_body,
        ActivityLogGetResponse {
            transactions: vec![],
            next_cursor: None,
        }
    )
}
