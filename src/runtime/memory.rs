use crate::from_le::FromLe;
use crate::runtime::error::RuntimeError;

pub struct Memory(Vec<u8>);

impl Memory {
    pub fn new(byte: Vec<u8>) -> Self {
        Self(byte)
    }

    pub fn load<T>(&mut self, addr: usize) -> Result<T, RuntimeError>
    where
        T: FromLe,
    {
        let size = std::mem::size_of::<T>();

        // NOTE 長さが足りないときは伸ばしてしまう。
        // メモリの最大長は決まっているのでそれ以上は伸びないように制限する必要がある
        while self.0.get(addr..addr + size).is_none() {
            let cap = self.0.capacity();
            self.0.extend(vec![0u8; cap]);
        }

        let buf = &self.0[addr..addr + size];

        Ok(T::from_le_bytes(buf))
    }
}
