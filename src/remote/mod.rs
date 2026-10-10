pub mod api;
pub mod cdn;
pub mod deploy;

use std::env;
use std::path::PathBuf;

pub fn get_env_deploy(path: &PathBuf) -> (String, String) {
    let _ = dotenvy::from_path(&path.join(".env"));

    let url = env::var("DEPLOY_URL").unwrap_or("http://localhost:8080".into());
    let token = env::var("DEPLOY_TOKEN").unwrap_or_default();

    (url, token)
}
