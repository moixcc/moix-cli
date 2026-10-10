use rhai::{CustomType, TypeBuilder};

#[derive(Debug, Clone)]
pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }

    fn hasher(&self) {}

    fn fetch(&self) {}

    fn kipu(&self) {}
}

impl CustomType for Context {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("Context");
    }
}
