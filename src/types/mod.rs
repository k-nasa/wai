use crate::opcode::Opcode;

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
        let x: u32 = x.into();
        ValueType::from(x as u8)
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

impl Into<i32> for VerUintN {
    fn into(self) -> i32 {
        self.0 as i32
    }
}

impl Into<usize> for VerUintN {
    fn into(self) -> usize {
        self.0 as usize
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

#[derive(Debug, Clone, PartialEq)]
pub struct ExportSection {
    pub entries: Vec<ExportEntry>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportEntry {
    pub field_str: String,
    pub kind: ExternalKind,
    pub index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExternalKind {
    Function,
    Table,
    Memory,
    Global,
    Unknown,
}

impl From<VerUintN> for ExternalKind {
    fn from(x: VerUintN) -> Self {
        use ExternalKind::*;

        match x.0 as u8 {
            0x00 => Function,
            0x01 => Table,
            0x02 => Memory,
            0x03 => Global,
            _ => Unknown,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CodeSection {
    pub(crate) bodies: Vec<FunctionBody>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionBody {
    pub(crate) locales: Vec<LocalEntry>,
    pub(crate) code: Vec<Opcode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalEntry {
    pub(crate) count: u32,
    pub(crate) value_type: ValueType,
}
