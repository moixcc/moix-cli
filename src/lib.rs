pub mod api;
pub mod local;
pub mod remote;
pub mod utils;

pub fn help() {
    println!(
        r#"moix OPTION [ARGS]

Options:
  init   - Create a directory with basic files
  build  - Generates a compressed file of dist/index.html
  dev    - Start a web server with port http://localhost:8080"#
    );
}
