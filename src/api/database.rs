use rhai::{CustomType, TypeBuilder};
use rusqlite::Connection;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct ApiDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl ApiDatabase {
    pub fn new(path: PathBuf) -> Self {
        let conn = Connection::open(path).expect("Error db_path");

        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl CustomType for ApiDatabase {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("Database");
    }
}
