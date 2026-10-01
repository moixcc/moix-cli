use rhai::{CustomType, TypeBuilder};

#[derive(Debug, Clone)]
pub struct Response {
    status: i32,
    data: Vec<u8>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: 0,
            data: Vec::new(),
        }
    }

    pub fn response(&self) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
        tiny_http::Response::from_data(self.data.clone()).with_status_code(self.status)
    }

    fn ok(&mut self, text: &str) -> Self {
        self.status = 200;
        self.data = text.as_bytes().to_vec();

        self.clone()
    }

    fn error(&mut self, text: &str, status: i64) -> Self {
        self.status = status as i32;
        self.data = text.as_bytes().to_vec();

        self.clone()
    }
}

impl CustomType for Response {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Response")
            .with_fn("ok", Self::ok)
            .with_fn("error", Self::error);
    }
}
