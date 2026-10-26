use crate::remote::get_env_deploy;
use reqwest::{self, StatusCode};
use std::{fs, path::PathBuf};

pub async fn handle(path: PathBuf) {
    let index_file = path.join("index.min.html");
    let index_bytes = match fs::read(&index_file) {
        Ok(bytes) => bytes,
        _ => {
            println!("Error {}", &index_file.display());
            return;
        }
    };
    let (url, token) = get_env_deploy(&path);
    let (status, text) = put_index_bytes(format!("{url}/app/index"), token, index_bytes).await;

    println!("{} | {} {}", index_file.display(), status, text);
}

pub async fn put_index_bytes(url: String, token: String, body: Vec<u8>) -> (StatusCode, String) {
    let client = reqwest::Client::new();
    let result = client
        .put(url)
        .bearer_auth(token)
        .header("Content-Type", "application/octet-stream")
        .body(body)
        .send()
        .await;

    match result {
        Ok(response) => (response.status(), response.text().await.unwrap()),
        Err(error) => (StatusCode::BAD_REQUEST, error.to_string()),
    }
}
