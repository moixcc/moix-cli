use moix_cli::remote;
use reqwest::StatusCode;

#[tokio::test]
async fn remote_put() {
    let (status, _text) = remote::put_bytes(
        "https://httpbin.org/put".into(),
        "TOKEN".into(),
        "text/plain".into(),
        vec![],
    )
    .await;

    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn remote_delete() {
    let (status, _text) = remote::delete("https://httpbin.org/delete".into(), "TOKEN".into()).await;

    assert_eq!(status, StatusCode::OK);
}
