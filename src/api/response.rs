use rhai::{CustomType, Dynamic, TypeBuilder};
use serde_json::Value;
use std::collections::BTreeMap;
use std::io::Cursor;
use tiny_http::Header;
use tiny_http::Response as ClientResponse;

#[derive(Debug, Clone)]
pub struct Response {
    status: i32,
    headers: BTreeMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: 500,
            headers: BTreeMap::new(),
            body: Vec::new(),
        }
    }

    pub fn response(&self) -> ClientResponse<Cursor<Vec<u8>>> {
        let mut response =
            ClientResponse::from_data(self.body.clone()).with_status_code(self.status);

        for (key, val) in self.headers.iter() {
            response.add_header(Header::from_bytes(key.as_bytes(), val.as_bytes()).unwrap());
        }

        response
    }

    fn ok(&mut self, text: &str) -> Self {
        self.status = 200;
        self.body = text.as_bytes().to_vec();

        self.clone()
    }

    fn error(&mut self, text: &str, status: i64) -> Self {
        self.status = status as i32;
        self.body = text.as_bytes().to_vec();

        self.clone()
    }

    fn json(&mut self, data: Dynamic) -> Self {
        if let Ok(json_val) = rhai::serde::from_dynamic::<Value>(&data) {
            self.status = 200;
            self.headers
                .insert("content-type".into(), "application/json".into());
            self.body = json_val.to_string().as_bytes().to_vec();
        }

        self.clone()
    }

    fn header(&self, name: &str) {}
}

impl CustomType for Response {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Response")
            .with_fn("ok", Self::ok)
            .with_fn("error", Self::error)
            .with_fn("json", Self::json);
    }
}
