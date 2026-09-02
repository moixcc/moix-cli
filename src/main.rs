mod local;
mod remote;
mod utils;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 3 {
        return help();
    }

    let option = args.get(1).unwrap();
    let path = match args.get(2) {
        Some(p) => std::path::PathBuf::from(p),
        _ => return help(),
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
            help();
        }
    }
}

fn help() {
    println!(r#"moix OPTION PATH

Options:
  init  - Create a directory with basic files.
  build - Generates a compressed file of dist/index.html
  dev   - Start a web server with port 8080.
"#);
}

pub fn exit() {
    std::process::exit(0);
}
