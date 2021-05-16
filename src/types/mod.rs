#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ValueType {
    I32,
    I64,
    F32,
    F64,
    Unknown,
}

impl From<u8> for ValueType {
    fn from(x: u8) -> Self {
        use ValueType::*;

        match x {
            0x7f => I32,
            0x02 => I64,
            0x03 => F32,
            0x04 => F64,
            _ => Unknown,
        }
    }
}

impl From<VerUintN> for ValueType {
    fn from(x: VerUintN) -> Self {
        use ValueType::*;

        match x.0 as u8 {
            0x7f => I32,
            0x02 => I64,
            0x03 => F32,
            0x04 => F64,
            _ => Unknown,
        }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub struct TypeSection {
    pub(crate) entries: Vec<FuncType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncType {
    pub(crate) params: Vec<ValueType>,
    pub(crate) returns: Vec<ValueType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSection {
    pub types: Vec<u32>,
}
