use rhai::{CustomType, Dynamic, ImmutableString, TypeBuilder};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Request {
    method: String,
    headers: BTreeMap<String, String>,
    body: Vec<u8>,
}

impl Request {
    pub fn new(method: String, body: Vec<u8>) -> Self {
        let headers = BTreeMap::new();

        Self {
            method,
            headers,
            body,
        }
    }

    fn method(&mut self) -> String {
        self.method.clone()
    }

    fn text(&mut self) -> ImmutableString {
        String::from_utf8_lossy(&self.body).to_string().into()
    }

    fn json(&mut self) -> Dynamic {
        if let Ok(json_val) = serde_json::from_slice::<Value>(&self.body) {
            rhai::serde::to_dynamic(json_val).unwrap_or_default()
        } else {
            Dynamic::UNIT
        }
    }

    fn header(&self, name: &str) {}
}

impl CustomType for Request {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Request")
            .with_get("method", Self::method)
            .with_get("text", Self::text)
            .with_get("json", Self::json);
    }
}
