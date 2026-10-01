use rhai::{CustomType, TypeBuilder};

#[derive(Debug, Clone)]
pub struct Request {
    method: String,
}

impl Request {
    pub fn new(method: String) -> Self {
        Self { method }
    }

    fn method(&mut self) -> String {
        self.method.clone()
    }
}

impl CustomType for Request {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Request")
            .with_get("method", Self::method);
    }
}
