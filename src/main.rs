use embassy_executor::Spawner;
use moix_cli::*;

#[embassy_executor::main]
async fn main(_spawn: Spawner) {
    let arguments: Vec<String> = std::env::args().collect();
    let option = arguments.get(1).map(|arg| arg.as_str());
    let path = arguments.get(2).map(|arg| arg.as_str());

    match (option, path) {
        // Local
        // "dev" => local::server::handle(path),
        // Remote
        // "deploy" => remote::deploy::handle(path),
        // "cdn" => remote::cdn::handle(path, args),
        // "api" => remote::api::handle(path, args),
        // Utils
        // "build" => utils::build::handle(path),
        (Some("init"), Some(path_str)) => utils::init::handle(path_str),
        (Some("b64"), Some(path_str)) => utils::b64::encode_path(path_str),
        _ => help(),
    }

    exit();
}
