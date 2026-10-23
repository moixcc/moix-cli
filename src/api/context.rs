use rhai::{CustomType, TypeBuilder};

#[derive(Debug, Clone)]
pub struct ApiContext {}

impl ApiContext {
    pub fn new() -> Self {
        Self {}
    }
}

impl CustomType for ApiContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("Context");
    }
}
