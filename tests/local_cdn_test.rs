use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use moix_cli::local::server;
use std::path::PathBuf;
use tower::ServiceExt;

#[tokio::test]
async fn cdn_get_data() {
    let app = server::router(PathBuf::from("./example"));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/cdn/data.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK)
}
