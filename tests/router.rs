use std::usize;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use axum_server_template::get_router;
use tower::ServiceExt;

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
