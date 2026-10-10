use rhai::{CustomType, TypeBuilder};

#[derive(Debug, Clone)]
pub struct Hasher {}

impl Hasher {
    pub fn new() -> Self {
        Self {}
    }
}

impl CustomType for Hasher {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("Hasher");
    }
}
