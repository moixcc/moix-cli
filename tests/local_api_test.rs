use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use moix_cli::local::server;
use std::path::PathBuf;
use tower::ServiceExt;

#[tokio::test]
async fn api_hello() {
    let app = server::router(PathBuf::from("./example"));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/hello")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body = response.into_body();
    let bytes = to_bytes(body, usize::MAX).await.unwrap();
    let body_str = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body_str, "Hello MOIX!");
}

#[tokio::test]
async fn api_json() {}

#[tokio::test]
async fn api_methods() {}

#[tokio::test]
async fn api_form_data() {}
