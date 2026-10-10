use rhai::{CustomType, TypeBuilder};

#[derive(Debug, Clone)]
pub struct Form {}

impl Form {
    pub fn new() -> Self {
        Self {}
    }
}

impl CustomType for Form {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("Form");
    }
}
