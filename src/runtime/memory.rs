use crate::from_le::FromLe;
use crate::runtime::error::RuntimeError;

pub struct Memory(Vec<u8>);

impl Memory {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn load<T>(&self, addr: usize) -> Result<T, RuntimeError>
    where
        T: FromLe,
    {
        let size = std::mem::size_of::<T>();
        let buf = &self.0[addr..addr + size];

        Ok(T::from_le_bytes(buf))
    }
}
