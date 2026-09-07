use std::fs;
use tiny_http::{Header, Method, Response};

#[derive(Debug, Default, serde::Deserialize)]
struct Config {
    mimetype: std::collections::HashMap<String, String>,
}

pub fn handle(path: std::path::PathBuf) {
    println!("moix dev {}\nhttp://localhost:8080", path.display());

    // Starting
    let server = tiny_http::Server::http("0.0.0.0:8080").unwrap();
    let index_path = path.join("index.bin");
    let config_str = fs::read_to_string(path.join("config.toml")).unwrap_or_default();
    let config = toml::from_str::<Config>(&config_str).unwrap_or_default();

    // Requests
    for request in server.incoming_requests() {
        let bytes: Vec<u8>;
        let mut content_type = "text/plain";
        let mut content_encoding: &str = "";

        match (request.method(), request.url()) {
            // No favicon.ico
            (Method::Get, "/favicon.ico") => {
                let _ = request.respond(Response::empty(404));
                continue;
            }
            // CDN
            (m, u) if m == &Method::Get && u.starts_with("/cdn/") => {
                let path_cdn = path.join(u.trim_start_matches('/'));

                bytes = fs::read(&path_cdn).expect("cdn error");

                if let Some(path_ext) = path_cdn.extension() {
                    if let Some(mimetype) =
                        config.mimetype.get(path_ext.to_str().unwrap_or_default())
                    {
                        content_type = mimetype;
                    };
                }
            }
            // All Request GET => index.bin
            (Method::Get, _) => {
                bytes = fs::read(&index_path).expect("index.bin error");
                content_type = "text/html; charset=UTF-8";
                content_encoding = "br";
            }
            // Request default
            _ => {
                let _ = request.respond(Response::empty(400));
                continue;
            }
        }

        // Response
        if bytes.is_empty() {
            continue;
        }

        let mut response = Response::from_data(bytes);

        response.add_header(Header::from_bytes(b"Content-Type", content_type.as_bytes()).unwrap());

        if !content_encoding.is_empty() {
            response.add_header(
                Header::from_bytes(b"Content-Encoding", content_encoding.as_bytes()).unwrap(),
            );
        }

        let _ = request.respond(response);
    }
}
