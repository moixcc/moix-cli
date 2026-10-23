use std::{fs, path::PathBuf};

pub const FILES: [&str; 8] = [
    "index.html",
    "css/1-style.css",
    "js/1-script.js",
    "icons.svg",
    "templates/not-found.html",
    "favicon.png",
    ".gitignore",
    ".env",
];

pub fn handle(path_str: &str) {
    let path = PathBuf::from(path_str);

    for file_name in FILES {
        let path_file = path.join(file_name);

        if let Some(path_dir) = path_file.parent() {
            let _ = fs::create_dir_all(path_dir);
        }

        if !path_file.exists() {
            let contents = get_contents(file_name);
            let _ = fs::write(path_file, contents);
        }
    }
}

fn get_contents(file_name: &str) -> &'static str {
    match file_name {
        "index.html" => "<html><head></head><body><main></main></body></html>",
        "css/1-style.css" => "*{font-size:inherit;color:inherit;}",
        "js/1-script.js" => "console.log('MOIX')",
        "icons.svg" => "<svg style=\"display:none\"></svg>",
        "templates/not-found.html" => {
            "<template id=\"template/not-found\"><h1>404</h1><h2>No Found</h2></template>"
        }
        ".gitignore" => "/index.min.html\n.env",
        ".env" => "DEPLOY_URL=\"http://localhost:8080\"\nDEPLOY_TOKEN=\"\"",
        _ => "",
    }
}
