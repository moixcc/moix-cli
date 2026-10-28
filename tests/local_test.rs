use axum::{body::Body, http::Request, http::StatusCode};
use moix_cli::local::server;
use std::path::PathBuf;
use tower::ServiceExt;

#[tokio::test]
async fn router_default() {
    let app = server::router(PathBuf::from("./example"));
    let response = app
        .oneshot(Request::post("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn router_favicon_ico() {
    let app = server::router(PathBuf::from("./example"));
    let response = app
        .oneshot(Request::get("/favicon.ico").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn router_index_min() {
    let app = server::router(PathBuf::from("./example"));
    let response = app
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
