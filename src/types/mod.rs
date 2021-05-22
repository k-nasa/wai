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

#[derive(Clone, Debug, PartialEq)]
pub struct VerUintN(u32);

impl From<u32> for VerUintN {
    fn from(x: u32) -> Self {
        VerUintN(x)
    }
}

impl From<i32> for VerUintN {
    fn from(x: i32) -> Self {
        VerUintN(x as u32)
    }
}

impl From<usize> for VerUintN {
    fn from(x: usize) -> Self {
        VerUintN(x as u32)
    }
}

impl From<VerUintN> for u32 {
    fn from(x: VerUintN) -> u32 {
        x.0
    }
}

impl From<VerUintN> for i32 {
    fn from(x: VerUintN) -> i32 {
        x.0 as i32
    }
}

impl From<VerUintN> for usize {
    fn from(x: VerUintN) -> usize {
        x.0 as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockType {
    I32,
    I64,
    F32,
    F64,
    Empty,
    Unknown,
}

impl From<u8> for BlockType {
    fn from(x: u8) -> Self {
        use BlockType::*;

        match x {
            0x7f => I32,
            0x02 => I64,
            0x03 => F32,
            0x04 => F64,
            0x40 => Empty,
            _ => Unknown,
        }
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
    pub(crate) code: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalEntry {
    pub(crate) count: u32,
    pub(crate) value_type: ValueType,
}

pub type Instruction = (Opcode, Operands);
pub type Operands = Vec<Operand>;

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    BlockType(BlockType),
    VerUintN(VerUintN),
}
