use rhai::{CustomType, TypeBuilder};

#[derive(Debug, Clone)]
pub struct Request {}

impl Request {
    pub fn new() -> Self {
        Self {}
    }
}

impl CustomType for Request {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("Request");
    }
}
