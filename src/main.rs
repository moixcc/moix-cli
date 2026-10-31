use moix_cli::*;
use std::env;

#[tokio::main]
async fn main() {
    let current_dir = env::current_dir().unwrap();
    let arguments: Vec<String> = env::args().collect();
    let option = arguments.get(1).map(|arg| arg.as_str());
    let arg_one = arguments.get(2).map(|arg| arg.as_str());
    let arg_two = arguments.get(3).map(|arg| arg.as_str());
    let arg_three = arguments.get(4).map(|arg| arg.as_str());

    match option {
        // local
        Some("dev") => local::server::handle(current_dir).await,
        // Remote
        Some("deploy") => remote::deploy::handle(current_dir).await,
        Some("cdn") if arg_one.is_some() && arg_two.is_some() => {
            remote::cdn::handle(current_dir, arg_one.unwrap(), arg_two.unwrap(), arg_three).await
        }
        Some("api") if arg_one.is_some() && arg_two.is_some() => {
            remote::api::handle(current_dir, arg_one.unwrap(), arg_two.unwrap(), arg_three).await
        }
        // Utils
        Some("init") => utils::init::handle(current_dir),
        Some("build") => utils::build::handle(current_dir),
        Some("b64") if arg_one.is_some() => utils::b64::encode_path(arg_one.unwrap()),
        _ => help(),
    }
}
