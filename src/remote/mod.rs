pub mod api;
pub mod cdn;
pub mod deploy;

use axum::http::StatusCode;
use std::{env, path::PathBuf};

pub fn get_env_deploy(path: &PathBuf) -> (String, String) {
    dotenvy::from_path(path.join(".env")).unwrap();

    let url = env::var("DEPLOY_URL").unwrap_or_default();
    let token = env::var("DEPLOY_TOKEN").unwrap_or_default();

    (url, token)
}

pub async fn put_bytes(
    url: String,
    token: String,
    content_type: String,
    body: Vec<u8>,
) -> (StatusCode, String) {
    let client = reqwest::Client::new();
    let result = client
        .put(url)
        .bearer_auth(token)
        .header("Content-Type", content_type)
        .body(body)
        .send()
        .await;

    match result {
        Ok(response) => (response.status(), response.text().await.unwrap()),
        Err(error) => (StatusCode::BAD_REQUEST, error.to_string()),
    }
}

pub async fn delete(url: String, token: String) -> (StatusCode, String) {
    let client = reqwest::Client::new();
    let result = client.delete(url).bearer_auth(token).send().await;

    match result {
        Ok(response) => (response.status(), response.text().await.unwrap()),
        Err(error) => (StatusCode::BAD_REQUEST, error.to_string()),
    }
}
