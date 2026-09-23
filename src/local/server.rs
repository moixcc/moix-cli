use std::fs;
use tiny_http::{Header, Method, Response};

pub fn handle(path: std::path::PathBuf) {
    // Config
    let config = crate::utils::Config::new(&path);
    let server = tiny_http::Server::http("0.0.0.0:8080").unwrap();
    let index_path = path.join("index.bin");

    println!("moix dev {}\n  >> http://localhost:8080\n", path.display());

    // Requests
    for request in server.incoming_requests() {
        let bytes: Vec<u8>;
        let content_type: String;
        let mut content_encoding: &str = "";
        let method = request.method();
        let uri = request.url();

        println!("{} {}", method.as_str(), uri);

        match (method, uri) {
            // No favicon.ico
            (Method::Get, "/favicon.ico") => {
                let _ = request.respond(Response::empty(404));
                continue;
            }
            // CDN
            (m, u) if m == &Method::Get && u.starts_with("/cdn/") => {
                let path_cdn = path.join(u.trim_start_matches('/'));

                bytes = fs::read(&path_cdn).unwrap_or_default();
                content_type = config.get_mimetype(&path_cdn);
            }
            // API
            (_, u) if u.starts_with("/api/") => {
                let _ = super::api::handle();
                let _ = request.respond(Response::empty(400));
                continue;
            }
            // All Request GET => index.bin
            (Method::Get, _) => {
                bytes = fs::read(&index_path).unwrap_or_default();
                content_type = "text/html; charset=UTF-8".into();
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
