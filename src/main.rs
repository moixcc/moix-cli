use moix_cli::*;

#[tokio::main]
async fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let option = arguments.get(1).map(|arg| arg.as_str());
    let path = arguments.get(2).map(|arg| arg.as_str());
    let action = arguments.get(3).map(|arg| arg.as_str());
    let uri = arguments.get(4).map(|arg| arg.as_str());

    match (option, path) {
        // local
        (Some("dev"), Some(path_str)) => local::server::handle(path_str).await,
        // Remote
        (Some("deploy"), Some(path_str)) => remote::deploy::handle(path_str).await,
        (Some("cdn"), Some(path_str)) => remote::cdn::handle(path_str, action, uri),
        (Some("api"), Some(path_str)) => remote::api::handle(path_str, action, uri),
        // Utils
        (Some("init"), Some(path_str)) => utils::init::handle(path_str),
        (Some("build"), Some(path_str)) => utils::build::handle(path_str),
        (Some("b64"), Some(path_str)) => utils::b64::encode_path(path_str),
        _ => help(),
    }
}
