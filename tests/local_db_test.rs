use std::path::PathBuf;

use moix_cli::api::database::{ApiDatabase, Entry};

#[test]
fn db_kipu() {
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
}
