use rhai::{CustomType, TypeBuilder};

#[derive(Debug, Clone)]
pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }
}

impl CustomType for Context {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("Context");
    }
}
