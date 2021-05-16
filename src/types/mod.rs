#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ValueType {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

pub struct VerUintN(u32);

impl From<u32> for VerUintN {
    fn from(x: u32) -> Self {
        VerUintN(x)
    }
}

impl Into<u32> for VerUintN {
    fn into(self) -> u32 {
        self.0
    }
}
