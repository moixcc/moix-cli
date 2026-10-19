use std::{fs, path::PathBuf};

pub fn handle(path_str: &str) {
    let path = PathBuf::from(path_str);

    // Contents
    let contents_favicon = read_favicon(path.join("favicon.png"));
    let contents_css = read_contents(path.join("css"), "css");
    let contents_js = read_contents(path.join("js"), "js");
    let contents_icons = fs::read(path.join("icons.svg")).unwrap_or_default();
    let contents_templates = read_contents(path.join("templates"), "html");
    let contents_index = fs::read(path.join("index.html")).unwrap_or_default();

    // Append contents
    let mut head: Vec<u8> = vec![];
    let mut body: Vec<u8> = vec![];

    //-- Favicon
    head.extend(b"<link rel=\"icon\" type=\"image/png\" href=\"data:image/png;base64,");
    head.extend(contents_favicon);
    head.extend(b"\"/>");
    //-- CSS
    head.extend(b"<style>");
    head.extend(contents_css);
    head.extend(b"</style>");
    //-- Icons
    body.extend(contents_icons);
    //-- JavaScript
    body.extend(b"<script type=\"module\">");
    body.extend(contents_js);
    body.extend(b"</script>");
    //-- Templates
    body.extend(contents_templates);

    let contents_index = append_contents(&contents_index, b"</head>", &head);
    let contents_index = append_contents(&contents_index, b"</body>", &body);

    // index.min.html
    let index_min = path.join("index.min.html");
    let _ = fs::write(&index_min, contents_index);

    println!("{}", index_min.display());
}

fn read_contents(path: PathBuf, ext: &str) -> Vec<u8> {
    let mut contents: Vec<u8> = Vec::new();

    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            let sub_contents = read_contents(path, ext);
            contents.extend(sub_contents);
        }
    } else {
        if path.extension().unwrap() == ext {
            let bytes = fs::read(path).unwrap_or_default();
            contents.extend(bytes);
        }
    }

    contents
}

fn append_contents(base: &[u8], pattern: &[u8], contents: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut size = 0;

    while size < base.len() {
        if base[size..].starts_with(pattern) {
            result.extend_from_slice(contents);
            result.extend_from_slice(pattern);
            size += pattern.len();
        } else {
            result.push(base[size]);
            size += 1;
        }
    }

    result
}

fn read_favicon(path: PathBuf) -> Vec<u8> {
    if let Some(b64_str) = crate::utils::b64::from_path(&path) {
        return b64_str.as_bytes().to_vec();
    }

    vec![]
}
