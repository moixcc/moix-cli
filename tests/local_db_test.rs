use moix_cli::api::database::{ApiDatabase, Entry};
use rhai::{Dynamic, Map};
use std::path::PathBuf;

#[test]
fn db_entry() {
    let mut db = ApiDatabase::new(PathBuf::from("./example/data/test.sqlite"));

    let mut db_test = db.space("test");

    // Entry
    let id = db_test.set_entry(0, 1, "context");

    assert_ne!(id.to_string(), "");

    let my_entry = db_test.get_entry(&id.to_string());

    assert_eq!(
        my_entry.cast::<Entry>(),
        Entry {
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

    // Context<ids>

    // Reference<ids>
}
