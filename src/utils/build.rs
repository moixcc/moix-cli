use std::{fs, io::Write, path::PathBuf};

pub fn handle(path: PathBuf) {
    println!("moix build {}", path.display());

    // CSS
    let mut head: Vec<u8> = "<style>".into();
    head.extend(read_dir_ext(path.join("dist/css"), "css"));
    head.extend(b"</style>\n");

    // favicon.png
    if path.join("dist/favicon.png").exists() {
        let mut favicon: Vec<u8> =
            r#"<link rel="icon" type="image/png" href="data:image/png;base64,"#.into();

        if let Some(b64_str) = crate::utils::b64::from_path(&path.join("dist/favicon.png")) {
            favicon.extend(b64_str.as_bytes());
        }

        favicon.extend(b"\"/>");
        head.extend(favicon);
    }

    head.extend(b"\n</head>");

    // JavaScript
    let mut body: Vec<u8> = "<script type=\"module\">\n".into();
    body.extend(read_dir_ext(path.join("dist/js"), "js"));
    body.extend(b"</script>\n");

    // Icons
    body.extend(fs::read(path.join("dist/icons.svg")).unwrap_or_default());

    // Templates
    body.extend(read_dir_ext(path.join("templates"), "html"));

    body.extend(b"\n</body>");

    // Replace
    let contents = fs::read(path.join("dist/index.html")).unwrap_or_default();
    let contents = replace(&contents, b"</head>", &head);
    let contents = replace(&contents, b"</body>", &body);

    // Minify
    let mut minify = html_minifier::HTMLMinifier::new();
    let _ = minify.digest(&contents);

    // Compress
    let mut writer = brotli::CompressorWriter::new(Vec::new(), 0, 11, 24);
    let _ = writer.write_all(minify.get_html());

    // index.bin
    fs::write(path.join("index.bin"), writer.into_inner()).unwrap();
}

fn replace(base: &[u8], pattern: &[u8], content: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < base.len() {
        if base[i..].starts_with(pattern) {
            result.extend_from_slice(content);
            i += pattern.len();
        } else {
            result.push(base[i]);
            i += 1;
        }
    }

    result
}

fn read_dir_ext(path_dir: PathBuf, ext: &str) -> Vec<u8> {
    let mut contents = Vec::new();

    for entry in walkdir::WalkDir::new(path_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some(ext) {
            let bytes = fs::read(path).expect(&path.to_string_lossy());
            contents.extend(bytes);
        }
    }

    contents
}
