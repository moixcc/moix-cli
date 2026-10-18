// use crate::utils::Config;
use std::fs::File;
use std::path::PathBuf;
use tiny_http::{Header, Method, Response, ResponseBox, Server};

const INDEX: &str = "index.bin";

#[embassy_executor::task]
pub async fn handle(path_str: String) {
    let path = PathBuf::from(path_str);
    let server = Server::http("0.0.0.0:8080").unwrap();

    // Requests
    for request in server.incoming_requests() {
        let method = request.method();
        let url = request.url();
        let (uri, query) = match url.split_once('?') {
            Some((p, q)) => (p, q),
            None => (url, ""),
        };

        println!("{method} {uri} {query}");

        let response = router(&path, &method, uri).await;
        let _ = request.respond(response);
    }
}

pub async fn router(path: &PathBuf, method: &Method, uri: &str) -> ResponseBox {
    match (method, uri) {
        // No favicon.ico
        (Method::Get, "/favicon.ico") => Response::empty(404).boxed(),

        // Response default for GET
        (Method::Get, _) => {
            if let Ok(index_bin) = File::open(path.join(INDEX)) {
                let content_type =
                    Header::from_bytes(b"Content-Type", b"text/html; charset=UTF-8").unwrap();
                let content_encoding = Header::from_bytes(b"Content-Encoding", b"br").unwrap();

                Response::from_file(index_bin)
                    .with_header(content_type)
                    .with_header(content_encoding)
                    .boxed()
            } else {
                Response::empty(404).boxed()
            }
        }

        // Response default
        _ => Response::empty(400).boxed(),
    }
}
//             // Remote Test
//             (Method::Put, u) if u == "/app/index" || u.starts_with("/cdn/") => {
//                 content_type = "text/plain".into();
//                 bytes = "OK".as_bytes().to_vec();
//             }
//             // CDN
//             (m, u) if m == &Method::Get && u.starts_with("/cdn/") => {
//                 let file = path.join(u.trim_start_matches('/'));

//                 if file.exists() {
//                     content_type = config.get_mimetype(&file);
//                     bytes = fs::read(&file)?;
//                 } else {
//                     request.respond(Response::empty(400))?;
//                     continue;
//                 }
//             }
//             // API
//             (_, u) if u.starts_with("/api/") => {
//                 let file = path
//                     .join(u.strip_prefix('/').unwrap_or(u))
//                     .with_extension("rhai");

//                 if file.exists() {
//                     if let Err(e) = crate::api::handle(request, file.clone()) {
//                         println!("{} | {:?}", file.display(), e);
//                     }
//                 } else {
//                     request.respond(Response::empty(400))?;
//                 }
//                 continue;
//             }
//             // DB
//             (_, u) if u.starts_with("/db/") => {
//                 request.respond(Response::empty(400))?;
//                 continue;
//             }
