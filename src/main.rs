mod local;
mod remote;
mod utils;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        return help();
    }

    let option = match args.get(1) {
        Some(o) => o,
        _ => return help(),
    };
    let path = match args.get(2) {
        Some(p) => std::path::PathBuf::from(p),
        _ => return help(),
    };

    return match option.as_str() {
        // Local
        "dev" => local::server::handle(path),
        // Remote
        "deploy" => remote::deploy::handle(path),
        "cdn" => remote::cdn::handle(path, args),
        // Utils
        "init" => utils::init::handle(path),
        "build" => utils::build::handle(path),
        "b64" => utils::b64::handle(path),
        _ => help(),
    };
}

fn help() -> anyhow::Result<()> {
    println!(
        r#"moix OPTION PATH

Options:
  init   - Create a directory with basic files
  build  - Generates a compressed file of dist/index.html
  dev    - Start a web server with port http://localhost:8080"#
    );

    Ok(())
}

pub fn exit() {
    std::process::exit(0);
}
