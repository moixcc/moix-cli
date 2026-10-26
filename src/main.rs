use moix_cli::*;
use std::env;

#[tokio::main]
async fn main() {
    let current_dir = env::current_dir().unwrap();
    let arguments: Vec<String> = env::args().collect();
    let option = arguments.get(1).map(|arg| arg.as_str());
    let path = arguments.get(2).map(|arg| arg.as_str());

    match option {
        // local
        Some("dev") => local::server::handle(current_dir).await,
        // Remote
        Some("deploy") => remote::deploy::handle(current_dir).await,
        // ! cdn
        // ! api
        // Utils
        Some("init") => utils::init::handle(current_dir),
        Some("build") => utils::build::handle(current_dir),
        Some("b64") if path.is_some() => utils::b64::encode_path(path.unwrap()),
        _ => help(),
    }
}
