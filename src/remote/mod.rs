pub mod cdn;
pub mod deploy;
use std::env;

pub fn get_env_deploy(path: &std::path::PathBuf) -> (String, String) {
    dotenvy::from_path(&path.join(".env")).expect(".env error");

    let url = env::var("DEPLOY_URL").expect("deploy_url error");
    let token = env::var("DEPLOY_TOKEN").expect("deploy_token error");

    println!("Load VARS");

    (url, token)
}
