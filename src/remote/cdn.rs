use crate::{
    remote::{delete, get_env_deploy, put_bytes},
    utils::Config,
};
use reqwest::StatusCode;
use std::{fs, path::PathBuf};

/// moix cdn {put | delete} /[uri..] [./file]
pub async fn handle(path: PathBuf, action: &str, uri: &str, path_file: Option<&str>) {
    let (url, token) = get_env_deploy(&path);
    let url = format!("{}/cdn/{}", url, uri);

    let (status, text) = match action {
        "put" if path_file.is_some() => {
            let path_file = PathBuf::from(path_file.unwrap());
            let config = Config::new(&path);
            let content_type = config.get_mimetype(&path_file);
            let body = match fs::read(&path_file) {
                Ok(bytes) => bytes,
                _ => {
                    println!("Error {}", path_file.display());
                    return;
                }
            };

            put_bytes(url.clone(), token, content_type, body).await
        }
        "delete" => delete(url.clone(), token).await,
        _ => (StatusCode::BAD_REQUEST, "Error".into()),
    };

    println!("{} | {} {}", url, status, text);
}
