use std::fs;

pub fn handle(path: std::path::PathBuf) -> anyhow::Result<()> {
    fs::create_dir_all(&path)?;

    for dir_name in ["api", "data", "templates", "dist/css", "dist/js"] {
        let path_dir = path.join(dir_name);
        fs::create_dir_all(path_dir)?;
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
            fs::write(path_file, get_content(file_name))?;
        }
    }

    Ok(())
}

fn get_content(file_name: &str) -> &'static str {
    match file_name {
        ".env" => "DEPLOY_URL=\"https://api.moix.cc\"\nDEPLOY_TOKEN=\"\"",
        ".gitignore" => "index.bin\ndata\n.env",
        "dist/icons.svg" => "<svg style=\"display:none;\"></svg>",
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
