use moix_cli::remote::deploy;
use reqwest::StatusCode;

#[tokio::test]
async fn deploy_index_min() {
    let (status, _text) =
        deploy::put_index_bytes("https://httpbin.org/put".into(), "TOKEN".into(), vec![]).await;

    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn cdn_put_remote() {}

#[tokio::test]
async fn api_put_remote() {}
