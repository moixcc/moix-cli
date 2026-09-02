use base64::Engine;
use std::path::PathBuf;

pub fn handle(mut path: PathBuf) {
    println!("moix b64 {}", path.display());

    if let Some(contents) = from_path(&path) {
        path.add_extension("txt");

        std::fs::write(path, contents.as_str()).unwrap();
    }
}

pub fn from_path(path: &PathBuf) -> Option<String> {
    if let Ok(bytes) = std::fs::read(&path) {
        return Some(base64::engine::general_purpose::STANDARD.encode(&bytes));
    }

    None
}
