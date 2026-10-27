use crate::remote::{get_env_deploy, put_bytes};
use std::{fs, path::PathBuf};

/// moix deploy
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
    let (status, text) = put_bytes(
        format!("{}/app/index", url),
        token,
        "application/octet-stream".into(),
        index_bytes,
    )
    .await;

    println!("{} | {} {}", index_file.display(), status, text);
}
