use crate::runtime::error::RuntimeError;
use crate::types::RuntimeValue;

pub struct Memory(Vec<u8>);

impl Memory {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn load<T>(&self, addr: u32) -> Result<RuntimeValue, RuntimeError>
    where
        T: From<RuntimeValue>,
    {
        todo!()
    }
}
