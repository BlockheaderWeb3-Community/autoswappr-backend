use axum::{
    body::Body,
    extract::Query,
    http::{Request, StatusCode},
};

use crate::helpers::*;

use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}

#[tokio::test]
async fn test_log_retrieval() {
    let app = TestApp::new().await; // 2024-11-24T10:30:00Z
    let req = Request::get("/log_retrieval?cursor=2024-11-28T12:02:49Z&limit=10")
        .body(Body::empty())
        .unwrap();
    let resp = app.request(req).await;
    let headers = resp.headers().clone();
    println!("{:#?}", resp.body());

    assert_eq!(resp.status(), StatusCode::OK);
    assert!(headers.get("x-request-id").is_some());
    assert_eq!(headers.get("access-control-allow-origin").unwrap(), "*");
    assert!(headers.get("vary").is_some());
}
