use axum::{
    body::{Body, Bytes},
    http::{HeaderName, HeaderValue, Response, StatusCode},
};
use rhai::{CustomType, Dynamic, TypeBuilder};
use serde_json::Value;
use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, Clone)]
pub struct ApiResponse {
    status: u16,
    headers: BTreeMap<String, String>,
    body: Bytes,
}

impl ApiResponse {
    pub fn new() -> Self {
        Self {
            status: 500,
            headers: BTreeMap::new(),
            body: Bytes::new(),
        }
    }

    pub fn response(&self) -> Response<Body> {
        let status_code = StatusCode::from_u16(self.status).unwrap_or_default();
        let mut response = Response::builder()
            .status(status_code)
            .body(Body::from(self.body.clone()))
            .unwrap();
        let headers = response.headers_mut();

        for (key, value) in self.headers.iter() {
            let name = HeaderName::from_str(key).unwrap();
            let value = HeaderValue::from_str(value).unwrap();

            headers.insert(name, value);
        }

        response
    }

    fn ok(&mut self, text: &str) -> Self {
        self.status = 200;
        self.body = Bytes::copy_from_slice(text.as_bytes());

        self.clone()
    }

    fn error(&mut self, text: &str, status: i64) -> Self {
        self.status = status as u16;
        self.body = Bytes::copy_from_slice(text.as_bytes());

        self.clone()
    }

    fn json(&mut self, data: Dynamic) -> Self {
        if let Ok(json_val) = rhai::serde::from_dynamic::<Value>(&data) {
            self.status = 200;
            self.headers
                .insert("content-type".into(), "application/json".into());
            self.body = Bytes::copy_from_slice(json_val.to_string().as_bytes());
        }

        self.clone()
    }

    fn header(&self, _name: &str) {}
}

impl CustomType for ApiResponse {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Response")
            .with_fn("ok", Self::ok)
            .with_fn("error", Self::error)
            .with_fn("json", Self::json);
    }
}
