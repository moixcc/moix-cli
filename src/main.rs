use moix_cli::*;

#[tokio::main]
async fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let option = arguments.get(1).map(|arg| arg.as_str());
    let path = arguments.get(2).map(|arg| arg.as_str());

    match (option, path) {
        // local
        (Some("dev"), Some(path_str)) => local::server::handle(path_str).await,
        // Utils
        (Some("init"), Some(path_str)) => utils::init::handle(path_str),
        (Some("build"), Some(path_str)) => utils::build::handle(path_str),
        (Some("b64"), Some(path_str)) => utils::b64::encode_path(path_str),
        _ => help(),
    }
}
