use std::path::PathBuf;

use moix_cli::api::database::ApiDatabase;

#[test]
fn db_kipu() {
    let mut db = ApiDatabase::new(PathBuf::from("./example/data/test.sqlite"));

    db = db.space("test");

    let id = db.set_entry(0, 1, "context");

    assert_ne!(id.to_string(), "");
}
