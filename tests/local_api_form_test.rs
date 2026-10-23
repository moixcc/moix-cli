use axum::{
    body::{Body, to_bytes},
    http::Request,
};
use moix_cli::local::server;
use serde_json::{Value, json};
use std::path::PathBuf;
use tower::ServiceExt;

const LIMIT: usize = usize::MAX;

#[tokio::test]
async fn api_form_data_urlencoded() {
    let app = server::router(PathBuf::from("./example"));
    let response = app
        .oneshot(
            Request::post("/api/form")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from("email=dev%40moix.cc"))
                .unwrap(),
        )
        .await
        .unwrap();

    let body_bytes = to_bytes(response.into_body(), LIMIT).await.unwrap();
    let data_val: Value = serde_json::from_slice(&body_bytes).unwrap();
    let data_ok = json!({
        "type": "application/x-www-form-urlencoded",
        "email": "dev@moix.cc"
    });

    assert_eq!(data_val, data_ok);
}

#[tokio::test]
async fn api_form_data_multipart() {
    let app = server::router(PathBuf::from("./example"));
    let boundary = "----------------boundary123";
    let body_data = format!(
        "--{0}\r\nContent-Disposition: form-data; name=\"email\"\r\n\r\ndev@moix.cc\r\n--{0}--\r\n",
        boundary
    );
    let content_type = format!("multipart/form-data; boundary={}", boundary);
    let response = app
        .oneshot(
            Request::post("/api/form")
                .header("Content-Type", content_type.clone())
                .body(Body::from(body_data))
                .unwrap(),
        )
        .await
        .unwrap();

    let body_bytes = to_bytes(response.into_body(), LIMIT).await.unwrap();
    let data_val: Value = serde_json::from_slice(&body_bytes).unwrap();
    let data_ok = json!({
        "type": &content_type,
        "email": "dev@moix.cc"
    });

    assert_eq!(data_val, data_ok);
}
