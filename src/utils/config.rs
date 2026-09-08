use std::path::PathBuf;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    mimetype: std::collections::HashMap<String, String>,
}

impl Config {
    pub fn new(path: &PathBuf) -> Self {
        let path_config = path.join("config.toml");
        let config_str = std::fs::read_to_string(&path_config).unwrap_or_default();

        println!("Load {}", path_config.display());

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
