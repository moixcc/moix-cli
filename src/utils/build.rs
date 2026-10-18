use brotli::CompressorWriter;
use html_minifier::HTMLMinifier;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

const QUALITY: u32 = 11; // Brotli Level
const LGWIN: u32 = 24; // Brotli Memory

pub fn handle(path_str: &str) {
    let path = PathBuf::from(path_str);

    // Contents
    let contents_favicon = read_favicon(path.join("dist/favicon.png"));
    let contents_css = read_dir_ext(path.join("dist/css"), "css");
    let contents_js = read_dir_ext(path.join("dist/js"), "js");
    let contents_icons = fs::read(path.join("dist/icons.svg")).unwrap_or_default();
    let contents_templates = read_dir_ext(path.join("templates"), "html");
    let contents_index = fs::read(path.join("dist/index.html")).unwrap_or_default();

    // Append contents
    let mut head: Vec<u8> = vec![];
    let mut body: Vec<u8> = vec![];
    //-- Favicon
    head.extend(b"<link rel=\"icon\" type=\"image/png\" href=\"data:image/png;base64,");
    head.extend(contents_favicon);
    head.extend(b"\"/>");
    //-- CSS
    head.extend(b"\n<style>");
    head.extend(contents_css);
    head.extend(b"</style>\n");
    //-- Icons
    body.extend(contents_icons);
    //-- JavaScript
    body.extend(b"\n<script type=\"module\">");
    body.extend(contents_js);
    body.extend(b"</script>\n");
    //-- Templates
    body.extend(contents_templates);

    let contents_index = append_contents(&contents_index, b"</head>", &head);
    let contents_index = append_contents(&contents_index, b"</body>", &body);

    // Minify
    let mut minify = HTMLMinifier::new();
    let _ = minify.digest(&contents_index);

    // Compress
    let mut writer = CompressorWriter::new(Vec::new(), 0, QUALITY, LGWIN);
    let _ = writer.write_all(minify.get_html());

    // index.bin
    let index_bin = path.join("index.bin");
    let _ = fs::write(&index_bin, writer.into_inner());
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

fn read_dir_ext(path_dir: PathBuf, ext: &str) -> Vec<u8> {
    let mut contents = Vec::new();

    for entry in WalkDir::new(path_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some(ext) {
            let bytes = fs::read(path).unwrap_or_default();
            contents.extend(bytes);
        }
    }

    contents
}

fn read_favicon(path: PathBuf) -> Vec<u8> {
    if let Some(b64_str) = crate::utils::b64::from_path(&path) {
        return b64_str.as_bytes().to_vec();
    }

    vec![]
}
