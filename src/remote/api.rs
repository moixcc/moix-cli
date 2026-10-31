use crate::remote::{delete, get_env_deploy, put_bytes};
use reqwest::StatusCode;
use std::{fs, path::PathBuf};

/// moix api {put | delete} /[uri...] [./path_file]
pub async fn handle(path: PathBuf, action: &str, uri: &str, path_file: Option<&str>) {
    let (url, token) = get_env_deploy(&path);
    let url = format!("{}/api/{}", url, uri);

    let (status, text) = match action {
        "put" if path_file.is_some() => {
            let path_file = PathBuf::from(path_file.unwrap());
            let body = match fs::read(&path_file) {
                Ok(bytes) => bytes,
                _ => {
                    println!("Error {}", path_file.display());
                    return;
                }
            };

            put_bytes(url.clone(), token, "application/octet-stream".into(), body).await
        }
        "delete" => delete(url.clone(), token).await,
        _ => (StatusCode::BAD_REQUEST, "Error".into()),
    };

    println!("{} | {} {}", url, status, text);
}
