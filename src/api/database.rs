use blake3::Hasher;
use chrono::Utc;
use rhai::{Array, Blob, CustomType, Dynamic, Map, TypeBuilder};
use rusqlite::{Connection, params};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use uuid::{NoContext, Timestamp, Uuid};

#[derive(Debug, Clone)]
pub struct ApiDatabase {
    conn: Arc<Mutex<Connection>>,
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub id: String,
    pub nature: i64,
    pub quantity: i64,
    pub context: String,
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

    pub fn set_entry(&mut self, nature: i64, quantity: i64, context: &str) -> Dynamic {
        if self.name.len() == 0 {
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
                "INSERT INTO {} (id, nature, quantity, context, created_at, hash) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                self.name
            ),
            params![id, nature, quantity, context, now.to_rfc3339(), hash,],
        );

        if result.is_ok() {
            id.into()
        } else {
            Dynamic::UNIT
        }
    }

    pub fn get_entry(&mut self, id: &str) -> Dynamic {
        let conn = self.conn.lock().unwrap();
        let entry: Entry = conn
            .query_one(
                &format!(
                    "SELECT id, nature, quantity, context FROM {} WHERE id = ?1 LIMIT 1",
                    self.name
                ),
                params![id],
                |row| {
                    Ok(Entry {
                        id: row.get(0).unwrap(),
                        nature: row.get(1).unwrap(),
                        quantity: row.get(2).unwrap(),
                        context: row.get(3).unwrap(),
                    })
                },
            )
            .unwrap();

        Dynamic::from(entry)
    }

    pub fn set_ref(&mut self, id: &str, reference: &str) {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now();

        conn.execute(
            &format!(
                "UPDATE {} SET reference = ?2, updated_at = ?3 WHERE id = ?1",
                self.name
            ),
            params![id, reference, now.to_rfc3339()],
        )
        .unwrap();
    }

    pub fn get_ref(&mut self, id: &str) -> Dynamic {
        let conn = self.conn.lock().unwrap();
        let reference: String = conn
            .query_one(
                &format!("SELECT reference FROM {} WHERE id = ?1 LIMIT 1", self.name),
                params![id],
                |row| row.get(0),
            )
            .unwrap();

        Dynamic::from(reference)
    }

    pub fn set_meta(&mut self, id: &str, metadata: Map) {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now();
        let metadata_map: HashMap<String, String> = metadata
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();
        let metadata_str = serde_json::to_string(&metadata_map).unwrap();

        conn.execute(
            &format!(
                "UPDATE {} SET metadata = ?2, updated_at = ?3  WHERE id = ?1",
                self.name
            ),
            params![id, metadata_str, now.to_rfc3339()],
        )
        .unwrap();
    }

    pub fn get_meta(&mut self, id: &str) -> Map {
        let conn = self.conn.lock().unwrap();
        let mut metadata = Map::new();

        let result: String = conn
            .query_one(
                &format!("SELECT metadata FROM {} WHERE id = ?1 LIMIT 1", self.name),
                params![id],
                |row| row.get(0),
            )
            .unwrap_or_default();

        if let Ok(data) = serde_json::from_str::<HashMap<String, String>>(&result) {
            for (key, value) in data {
                metadata.insert(key.into(), Dynamic::from(value));
            }
        };

        metadata
    }

    pub fn set_raw(&mut self, id: &str, raw: Blob) {}

    pub fn get_raw(&mut self, id: &str) -> Blob {
        Blob::new()
    }

    pub fn on_ctx(&mut self, context: &str) -> Array {
        Array::new()
    }

    pub fn on_ref(&mut self, reference: &str) -> Array {
        Array::new()
    }
}

impl CustomType for ApiDatabase {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Database")
            .with_fn("space", Self::space)
            .with_fn("set_entry", Self::set_entry)
            .with_fn("get_entry", Self::get_entry)
            .with_fn("set_ref", Self::set_ref)
            .with_fn("get_ref", Self::get_ref)
            .with_fn("set_meta", Self::set_meta)
            .with_fn("get_meta", Self::get_meta)
            .with_fn("set_raw", Self::set_raw)
            .with_fn("get_raw", Self::get_raw)
            .with_fn("on_ctx", Self::on_ctx)
            .with_fn("on_ref", Self::on_ref);
    }
}
