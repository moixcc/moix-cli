use std::fs;

pub fn handle(path: std::path::PathBuf) {
    println!("moix init {}", path.display());

    let _ = fs::create_dir_all(&path);

    for dir_name in ["api", "data", "templates", "dist/css", "dist/js"] {
        let _ = fs::create_dir_all(path.join(dir_name));
    }

    for file_name in [
        "dist/index.html",
        "dist/icons.svg",
        "dist/js/1_script.js",
        "dist/css/1_style.css",
        ".gitignore",
        ".env",
    ] {
        let path_file = path.join(file_name);

        if !path_file.exists() {
            let _ = fs::write(path_file, get_content(file_name));
        }
    }

    crate::exit();
}

fn get_content(file_name: &str) -> &'static str {
    match file_name {
        ".env" => "DEPLOY_URL=\"https://api.moix.cc\"\nDEPLOY_TOKEN=\"\"",
        ".gitignore" => "index.bin\ndata\n.env",
        "dist/icons.svg" => "<svg></svg>",
        "dist/css/1_style.css" => "body{text-align:center;}",
        "dist/js/1_script.js" => "console.log('WWW.MOIX.CC');",
        "dist/index.html" => {
            "<!doctype html><html><head></head>\
            <body><h1><a href=\"https://moix.cc\">MOIX</a>\
            </h1></body></html>"
        }
        _ => "",
    }
}
