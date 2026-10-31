use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use toml;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Config {
    mimetype: HashMap<String, String>,
}

impl Config {
    pub fn new(path: &PathBuf) -> Self {
        let path_config = path.join("config.toml");
        let config_str = fs::read_to_string(&path_config).unwrap_or_default();

        toml::from_str::<Config>(&config_str).unwrap_or_default()
    }

    pub fn get_mimetype(&self, path: &PathBuf) -> String {
        let default = String::from("text/plain");

        if let Some(ext) = path.extension() {
            if let Some(mimetype) = self.mimetype.get(ext.to_str().unwrap_or(&default)) {
                return mimetype.to_owned();
            };
        }

        default
    }
}
