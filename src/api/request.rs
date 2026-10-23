use axum::body::Bytes;
use axum::http::HeaderMap;
use rhai::{CustomType, Dynamic, ImmutableString, TypeBuilder};
use serde_json::Value;
use std::collections::HashMap;
use std::mem;

#[derive(Debug, Clone)]
pub struct ApiRequest {
    method: String,
    headers: HeaderMap,
    body: Bytes,
    form_data: HashMap<String, String>,
}

impl ApiRequest {
    pub fn new(
        method: String,
        headers: HeaderMap,
        body: Bytes,
        form_data: HashMap<String, String>,
    ) -> Self {
        Self {
            method,
            headers,
            body,
            form_data,
        }
    }

    fn method(&mut self) -> String {
        self.method.clone()
    }

    fn text(&mut self) -> ImmutableString {
        let body = mem::take(&mut self.body);

        String::from_utf8_lossy(&body).to_string().into()
    }

    fn json(&mut self) -> Dynamic {
        let body = mem::take(&mut self.body);

        if let Ok(json_val) = serde_json::from_slice::<Value>(&body) {
            rhai::serde::to_dynamic(json_val).unwrap_or_default()
        } else {
            Dynamic::UNIT
        }
    }

    fn header(&mut self, name: &str) -> ImmutableString {
        if let Some(value) = self.headers.get(name) {
            return value.to_str().unwrap().into();
        }

        String::new().into()
    }

    fn form(&mut self, name: &str) -> ImmutableString {
        if let Some(value) = self.form_data.get(name) {
            return value.into();
        }

        String::new().into()
    }
}

impl CustomType for ApiRequest {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Request")
            .with_get("method", Self::method)
            .with_get("text", Self::text)
            .with_get("json", Self::json)
            .with_fn("header", Self::header)
            .with_fn("form", Self::form);
    }
}
