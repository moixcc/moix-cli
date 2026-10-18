use embassy_executor::Spawner;
use moix_cli::*;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let arguments: Vec<String> = std::env::args().collect();
    let option = arguments.get(1).map(|arg| arg.as_str());
    let path = arguments.get(2).map(|arg| arg.as_str());
    let mut is_once = true;

    match (option, path) {
        // Local
        (Some("dev"), Some(path_str)) => {
            let token = local::server::handle(path_str.to_string()).expect("error server");
            let _ = spawner.spawn(token);

            is_once = false;

            println!("http://localhost:8080");
        }
        // Remote
        // "deploy" => remote::deploy::handle(path),
        // "cdn" => remote::cdn::handle(path, args),
        // "api" => remote::api::handle(path, args),
        // Utils
        (Some("build"), Some(path_str)) => utils::build::handle(path_str),
        (Some("init"), Some(path_str)) => utils::init::handle(path_str),
        (Some("b64"), Some(path_str)) => utils::b64::encode_path(path_str),
        _ => help(),
    }

    if is_once {
        exit();
    }
}
