use crate::remote::get_env_deploy;
use std::path::PathBuf;

/// moix api {put | delete} /[uri...] [./path_file]
pub async fn handle(path: PathBuf, _action: &str, _uri: &str, _path_file: Option<&str>) {
    let (_url, _token) = get_env_deploy(&path);
}
