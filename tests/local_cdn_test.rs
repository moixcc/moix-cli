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
        .oneshot(Request::get("/cdn/data.json").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK)
}

#[tokio::test]
async fn cdn_set_user() {}

#[tokio::test]
async fn cdn_del_user() {}
