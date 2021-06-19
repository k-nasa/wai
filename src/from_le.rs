pub trait FromLe {
    fn from_le_bytes(b: &[u8]) -> Self;
}

impl FromLe for u32 {
    fn from_le_bytes(byte: &[u8]) -> Self {
        let mut b: [u8; 4] = Default::default();
        b.copy_from_slice(&byte[0..4]);
        Self::from_le_bytes(b)
    }
}

impl FromLe for i32 {
    fn from_le_bytes(byte: &[u8]) -> Self {
        let mut b: [u8; 4] = Default::default();
        b.copy_from_slice(&byte[0..4]);
        Self::from_le_bytes(b)
    }
}
impl FromLe for u64 {
    fn from_le_bytes(byte: &[u8]) -> Self {
        let mut b: [u8; 8] = Default::default();
        b.copy_from_slice(&byte[0..8]);
        Self::from_le_bytes(b)
    }
}
impl FromLe for i64 {
    fn from_le_bytes(byte: &[u8]) -> Self {
        let mut b: [u8; 8] = Default::default();
        b.copy_from_slice(&byte[0..8]);
        Self::from_le_bytes(b)
    }
}
impl FromLe for f32 {
    fn from_le_bytes(byte: &[u8]) -> Self {
        let mut b: [u8; 4] = Default::default();
        b.copy_from_slice(&byte[0..4]);
        Self::from_le_bytes(b)
    }
}

impl FromLe for f64 {
    fn from_le_bytes(byte: &[u8]) -> Self {
        let mut b: [u8; 8] = Default::default();
        b.copy_from_slice(&byte[0..8]);
        Self::from_le_bytes(b)
    }
}

impl FromLe for usize {
    fn from_le_bytes(byte: &[u8]) -> Self {
        let mut b: [u8; 8] = Default::default();
        b.copy_from_slice(&byte[0..8]);
        Self::from_le_bytes(b)
    }
}
