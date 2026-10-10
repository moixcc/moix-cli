use rhai::{CustomType, TypeBuilder};

#[derive(Debug, Clone)]
pub struct Fetch {}

impl Fetch {
    pub fn new() -> Self {
        Self {}
    }
}

impl CustomType for Fetch {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("Fetch");
    }
}
