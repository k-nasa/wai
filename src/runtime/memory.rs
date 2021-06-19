use crate::from_le::FromLe;
use crate::runtime::error::RuntimeError;
use crate::types::RuntimeValue;

pub struct Memory(Vec<u8>);

impl Memory {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn load<T>(&self, addr: usize) -> Result<T, RuntimeError>
    where
        T: Into<RuntimeValue> + FromLe,
    {
        let size = std::mem::size_of::<T>();
        let buf = &self.0[addr..addr + size];

        Ok(T::from_le_bytes(buf))
    }
}
