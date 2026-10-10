use rhai::{CustomType, TypeBuilder};

#[derive(Debug, Clone)]
struct Kipu {}

impl Kipu {
    pub fn new() -> Self {
        Self {}
    }
}

impl CustomType for Kipu {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("Kipu");
    }
}
