use crate::module::Module;

pub struct Instance {
    module: Module,
}

impl Instance {
    pub fn new(module: Module) -> Self {
        Self { module }
    }

    pub fn invoke(&self, name: impl AsRef<str>) -> std::io::Result<()> {
        todo!()
    }
}
