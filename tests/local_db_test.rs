use axum::{
    body::{Body, to_bytes},
    http::Request,
};
use moix_cli::{
    api::database::{ApiDatabase, ApiEntry},
    local::server,
};
use rhai::{Dynamic, Map};
use std::path::PathBuf;
use tower::ServiceExt;

#[test]
fn db_entry() {
    let mut db = ApiDatabase::new(PathBuf::from("./example/data/test.sqlite"));

    let mut db_test = db.space("test");

    // Entry
    let id = db_test.set_entry(0, 1, "context");

    assert_ne!(id.to_string(), "");

    let my_entry = db_test.get_entry(&id.to_string());

    assert_eq!(
        my_entry.cast::<ApiEntry>(),
        ApiEntry {
            id: id.to_string(),
            nature: 0,
            quantity: 1,
            context: "context".to_string()
        }
    );

    // Reference
    db_test.set_ref(&id.to_string(), "reference");

    let my_ref = db_test.get_ref(&id.to_string());

    assert_eq!(my_ref.to_string(), "reference");

    // Metadata
    let mut metadata = Map::new();

    metadata.insert("content-type".into(), Dynamic::from("text/plain"));

    db_test.set_meta(&id.to_string(), metadata);

    let my_meta = db_test.get_meta(&id.to_string());

    assert_eq!(
        my_meta.get("content-type").unwrap().to_string(),
        "text/plain".to_string()
    );

    // Raw
    let raw = b"hello".to_vec();

    db_test.set_raw(&id.to_string(), raw.clone());

    let my_raw = db_test.get_raw(&id.to_string());

    assert_eq!(my_raw, raw);

    // Context<ids>
    let list_ctx = db_test.on_ctx("context");

    assert!(list_ctx.len() > 0);

    // Reference<ids>
    let list_ref = db_test.on_ref("reference");

    assert!(list_ref.len() > 0);
}

#[tokio::test]
async fn db_api_entry() {
    let app = server::router(PathBuf::from("./example"));
    let response = app
        .oneshot(Request::get("/api/db").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

    assert_eq!(&body_bytes[..], b"context");
}
