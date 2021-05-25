use crate::module::Module;

#[derive(Debug)]
pub struct Instance {
    module: Module,
}

impl Instance {
    pub fn new(module: Module) -> Self {
        Self { module }
    }

    pub fn invoke(&self, name: impl AsRef<str>) -> Result<(), RuntimeError> {
        let index = self.resolve_function_name(name.as_ref());
        let index = match index {
            None => return Err(RuntimeError::NotFound(name.as_ref().to_string())),
            Some(i) => i,
        };

        let instractions = &self
            .module
            .code_section
            .as_ref()
            .unwrap()
            .bodies
            .get(index)
            .unwrap();

        dbg!(instractions);
        todo!()
    }

    fn resolve_function_name(&self, name: impl AsRef<str>) -> Option<usize> {
        let export_section = &self.module.export_section;

        let exports = match export_section {
            None => return None,
            Some(e) => &e.entries,
        };

        let entry = exports.iter().find(|x| &x.field_str == name.as_ref());

        entry.and_then(|x| Some(x.index as usize))
    }
}

use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum RuntimeError {
    NotFound(String),
    IOError(std::io::Error),
}

impl Error for RuntimeError {}
impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RuntimeError::*;
        match self {
            NotFound(name) => write!(f, "'{}'' is not found", name),
            IOError(i) => write!(f, "io error: {}", i),
        }
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}
