mod local;
mod remote;
mod utils;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let option = args.get(1).unwrap();
    let path = match args.get(2) {
        Some(p) => std::path::PathBuf::from(p),
        _ => return println!("moix OPTION PATH"),
    };

    match option.as_str() {
        // Local
        "dev" => local::server::handle(path),
        // Remote
        "deploy" => remote::deploy::handle(path),
        // Utils
        "init" => utils::init::handle(path),
        "build" => utils::build::handle(path),
        "b64" => utils::b64::handle(path),
        _ => {
            println!("moix OPTION PATH")
        }
    }

    exit();
}

pub fn exit() {
    std::process::exit(0);
}
