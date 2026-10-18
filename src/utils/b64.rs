use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use std::path::PathBuf;

pub fn encode_bytes(bytes: &[u8]) -> String {
    STANDARD.encode(bytes)
}

pub fn from_path(path: &PathBuf) -> Option<String> {
    if let Ok(bytes) = std::fs::read(&path) {
        return Some(encode_bytes(&bytes));
    }

    None
}

pub fn encode_path(path_str: &str) {
    let mut path = PathBuf::from(path_str);

    if let Some(contents) = from_path(&path) {
        path.add_extension("txt");

        let _ = std::fs::write(&path, contents.as_str());

        println!("{}", path.display());
    }
}
