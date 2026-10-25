pub mod api;
pub mod cdn;
pub mod deploy;

use std::{env, path::PathBuf};

pub fn get_env_deploy(path: &PathBuf) -> (String, String) {
    dotenvy::from_path(path.join(".env")).unwrap();

    let url = env::var("DEPLOY_URL").unwrap_or_default();
    let token = env::var("DEPLOY_TOKEN").unwrap_or_default();

    (url, token)
}
