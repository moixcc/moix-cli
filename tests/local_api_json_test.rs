use axum::{
    body::{Body, to_bytes},
    http::{HeaderValue, Request, StatusCode},
};
use moix_cli::local::server;
use std::path::PathBuf;
use tower::ServiceExt;

const LIMIT: usize = usize::MAX;
const JSON: &str = r#"{"urls":["https://moix.cc","https://api.moix.cc"],"username":"moix"}"#;

#[tokio::test]
async fn api_json_text() {
    let app = server::router(PathBuf::from("./example"));

    let response = app
        .oneshot(Request::put("/api/json").body(Body::from(JSON)).unwrap())
        .await
        .unwrap();

    let body_bytes = to_bytes(response.into_body(), LIMIT).await.unwrap();

    assert_eq!(&body_bytes[..], JSON.as_bytes());
}

#[tokio::test]
async fn api_json_get() {
    let app = server::router(PathBuf::from("./example"));

    let response = app
        .oneshot(Request::get("/api/json").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        HeaderValue::from_bytes(b"application/json").unwrap()
    );

    let body_bytes = to_bytes(response.into_body(), LIMIT).await.unwrap();

    assert_eq!(&body_bytes[..], JSON.as_bytes());
}

#[tokio::test]
async fn api_json_post_error() {
    let app = server::router(PathBuf::from("./example"));

    let response = app
        .oneshot(Request::post("/api/json").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE);
}
