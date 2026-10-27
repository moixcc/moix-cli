use crate::{remote::get_env_deploy, utils::Config};
use std::path::PathBuf;

/// moix api {put | delete} /[uri...] [./path_file]
pub async fn handle(path: PathBuf, action: &str, _uri: &str, _path_file: Option<&str>) {
    let (url, token) = get_env_deploy(&path);
    let config = Config::new(&path);

    match action {
        "put" => (),
        "delete" => (),
        _ => {
            println!("moix api put [uri] [path_file]");
        }
    }
}
