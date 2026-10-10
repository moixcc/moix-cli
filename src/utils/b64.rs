use anyhow::Result;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use std::path::PathBuf;

pub fn handle(mut path: PathBuf) -> Result<()> {
    if let Some(contents) = from_path(&path) {
        path.add_extension("txt");

        std::fs::write(&path, contents.as_str())?;

        println!("{}", path.display());
    }

    Ok(())
}

pub fn from_path(path: &PathBuf) -> Option<String> {
    if let Ok(bytes) = std::fs::read(&path) {
        return Some(STANDARD.encode(&bytes));
    }

    None
}
