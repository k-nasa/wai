use crate::module::Module;

#[derive(Debug)]
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
