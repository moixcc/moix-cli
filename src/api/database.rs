use blake3::Hasher;
use chrono::Utc;
use rhai::{CustomType, Dynamic, TypeBuilder};
use rusqlite::Connection;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use uuid::{NoContext, Timestamp, Uuid};

#[derive(Debug, Clone)]
pub struct ApiDatabase {
    conn: Arc<Mutex<Connection>>,
    name: String,
}

impl ApiDatabase {
    pub fn new(path: PathBuf) -> Self {
        let conn = Connection::open(path).expect("Error db_path");

        Self {
            conn: Arc::new(Mutex::new(conn)),
            name: String::new(),
        }
    }

    pub fn space(&mut self, name: &str) -> Self {
        self.name = name.to_string();
        let conn = self.conn.lock().unwrap();

        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {name} (
                    id TEXT,
                    nature INTEGER,
                    quantity INTEGER,
                    context TEXT,
                    created_at TEXT, 
                    hash TEXT,
                    updated_at TEXT,
                    reference TEXT,
                    metadata TEXT, 
                    raw BLOB
                );"
            ),
            [],
        )
        .unwrap();

        self.clone()
    }

    pub fn set_entry(&mut self, nature: i8, quantity: u64, context: &str) -> Dynamic {
        let name = if self.name.len() > 0 {
            self.name.clone()
        } else {
            return Dynamic::UNIT;
        };

        if nature < -1 || nature > 1 || quantity == 0 || context.len() == 0 {
            return Dynamic::UNIT;
        }

        let now = Utc::now();
        let ts = Timestamp::from_unix(
            NoContext,
            now.timestamp() as u64,
            now.timestamp_subsec_nanos(),
        );
        let id = Uuid::new_v7(ts).to_string();
        let conn = self.conn.lock().unwrap();
        let hash = Hasher::new()
            .update(id.as_bytes())
            .update(&nature.to_le_bytes())
            .update(&quantity.to_le_bytes())
            .update(context.as_bytes())
            .update(now.to_rfc3339().as_bytes())
            .finalize()
            .to_string();

        let result = conn.execute(
            &format!(
                "INSERT INTO {name}(
                id, nature, quantity, context, created_at, hash
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6
            )"
            ),
            [
                id.clone(),
                nature.to_string(),
                quantity.to_string(),
                context.to_string(),
                now.to_rfc3339(),
                hash,
            ],
        );

        if result.is_ok() {
            id.into()
        } else {
            Dynamic::UNIT
        }
    }
}

impl CustomType for ApiDatabase {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Database")
            .with_fn("space", Self::space)
            .with_fn("set_entry", Self::set_entry);
    }
}
