use crate::utils::Config;
use anyhow::{Error, Result};
use std::fs;
use std::path::PathBuf;
use tiny_http::Server;
use tiny_http::{Header, Method, Response};

pub fn handle(path: PathBuf) -> Result<(), Error> {
    println!("http://localhost:8080");

    // Config
    let config = Config::new(&path);
    let server = Server::http("0.0.0.0:8080").unwrap();
    let index_path = path.join("index.bin");

    // Requests
    for request in server.incoming_requests() {
        let bytes: Vec<u8>;
        let content_type: String;
        let mut content_encoding: &str = "";
        let method = request.method();
        let url = request.url();
        let (uri, _query) = match url.split_once('?') {
            Some((p, q)) => (p, Some(q)),
            None => (url, None),
        };

        println!("{method} {uri}");

        match (method, uri) {
            // No favicon.ico
            (Method::Get, "/favicon.ico") => {
                request.respond(Response::empty(404))?;
                continue;
            }
            // Remote Test
            (Method::Put, u) if u == "/app/index" || u.starts_with("/cdn/") => {
                content_type = "text/plain".into();
                bytes = "OK".as_bytes().to_vec();
            }
            // CDN
            (m, u) if m == &Method::Get && u.starts_with("/cdn/") => {
                let path_cdn = path.join(u.trim_start_matches('/'));

                content_type = config.get_mimetype(&path_cdn);
                bytes = fs::read(&path_cdn)?;
            }
            // API
            (_, u) if u.starts_with("/api/") => {
                let file = path
                    .join(u.strip_prefix('/').unwrap_or(u))
                    .with_extension("rhai");
                if file.exists() {
                    if let Err(e) = crate::api::handle(request, file.clone()) {
                        println!("{} | {:?}", file.display(), e);
                    }
                } else {
                    request.respond(Response::empty(400))?;
                }
                continue;
            }
            // DB
            (_, u) if u.starts_with("/db/") => {
                request.respond(Response::empty(400))?;
                continue;
            }
            // All Request GET => index.bin
            (Method::Get, _) => {
                content_type = "text/html; charset=UTF-8".into();
                content_encoding = "br";
                bytes = fs::read(&index_path)?;
            }
            // Request default
            _ => {
                request.respond(Response::empty(400))?;
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

        request.respond(response)?;
    }

    Ok(())
}
