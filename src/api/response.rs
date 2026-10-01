use rhai::{CustomType, TypeBuilder};

#[derive(Debug, Clone)]
pub struct Response {
    pub data: Vec<u8>,
}

impl Response {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn response(&self) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
        tiny_http::Response::from_data(self.data.clone())
    }

    fn ok(&mut self, text: &str) -> Self {
        self.data = text.as_bytes().to_vec();

        self.clone()
    }
}

impl CustomType for Response {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("Response").with_fn("ok", Self::ok);
    }
}
