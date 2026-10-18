use moix_cli::local::server::router;
use std::path::PathBuf;
use tiny_http::Method;

#[pollster::test]
async fn server_default() {
    let response = router(&PathBuf::default(), &Method::Head, "/").await;

    assert_eq!(response.status_code(), 400);
}

#[pollster::test]
async fn server_favicon_ico() {
    let response = router(&PathBuf::default(), &Method::Get, "/favicon.ico").await;

    assert_eq!(response.status_code(), 404);
}

#[pollster::test]
async fn server_index_bin() {
    let path = PathBuf::from("./example");
    let response = router(&path, &Method::Get, "/").await;
    let headers: Vec<[&str; 2]> = response
        .headers()
        .iter()
        .map(|header| [header.field.as_str().as_str(), header.value.as_str()])
        .collect();

    assert_eq!(response.status_code(), 200);
    assert!(headers.contains(&["Content-Type", "text/html; charset=UTF-8"]));
    assert!(headers.contains(&["Content-Encoding", "br"]));
}
