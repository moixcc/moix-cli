use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use moix_cli::local::server;
use std::path::PathBuf;
use tower::ServiceExt;

const LIMIT: usize = usize::MAX;

#[tokio::test]
async fn api_none() {
    let app = server::router(PathBuf::from("./example"));
    let response = app
        .oneshot(Request::get("/api/none").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);

    let body_bytes = to_bytes(response.into_body(), LIMIT).await.unwrap();

    assert_eq!(&body_bytes[..], b"Function not found: on");
}

#[tokio::test]
async fn api_hello() {
    let app = server::router(PathBuf::from("./example"));
    let response = app
        .oneshot(
            Request::builder()
                .method("QUERY")
                .uri("/api/hello")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), LIMIT).await.unwrap();

    assert_eq!(&body_bytes[..], b"Hello MOIX!");
}

#[tokio::test]
async fn api_json() {}

#[tokio::test]
async fn api_methods() {}

#[tokio::test]
async fn api_form_data() {}
