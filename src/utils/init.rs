use std::fs;
use std::path::PathBuf;

const DIRS: [&str; 5] = ["api", "data", "templates", "dist/css", "dist/js"];

pub const FILES: [&str; 6] = [
    "dist/index.html",
    "dist/icons.svg",
    "dist/js/1-script.js",
    "dist/css/1-style.css",
    ".gitignore",
    ".env",
];

pub fn handle(path_str: &str) {
    let path = PathBuf::from(path_str);
    let _ = fs::create_dir_all(&path);

    for dir_name in DIRS {
        let path_dir = path.join(dir_name);
        let _ = fs::create_dir_all(path_dir);
    }

    for file_name in FILES {
        let path_file = path.join(file_name);

        if !path_file.exists() {
            let contents = get_contents(file_name);
            let _ = fs::write(path_file, &contents);
        }
    }
}

fn get_contents(file_name: &str) -> &'static str {
    match file_name {
        ".env" => "DEPLOY_URL=\"http://localhost:8080\"\nDEPLOY_TOKEN=\"\"",
        ".gitignore" => "index.bin\ndata\n.env",
        "dist/icons.svg" => "<svg style=\"display:none;\"></svg>",
        "dist/css/1-style.css" => "body{text-align:center;}",
        "dist/js/1-script.js" => "console.log('WWW.MOIX.CC');",
        "dist/index.html" => {
            "<!doctype html><html><head></head>\
                    <body><h1><a href=\"https://moix.cc\">MOIX</a>\
                    </h1></body></html>"
        }
        _ => "",
    }
}
