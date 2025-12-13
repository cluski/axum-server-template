use std::usize;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use axum_server_template::{get_router, start_server};
use tower::ServiceExt;

use rootcause::Report;

use crate::client::TestClient;

#[tokio::test]
async fn test_router() {
    let app = get_router();
    let req = Request::builder()
        .uri("/hello")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // 解析响应体并断言内容
    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    assert_eq!(body, "Hello, World!");
}

#[tokio::test]
async fn test_router_by_client() -> Result<(), Report> {
    tokio::spawn(start_server());
    let client = TestClient::new()?;
    let resp: serde_json::Value = client.get("/health").await?;
    assert_eq!(resp["status"], "healthy");

    Ok(())
}
