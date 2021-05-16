#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    Unexpected,
    Nop,
    Block,
    Loop,
    If,
    Else,
    End,
    Br,
    BrIf,
    BrTable,
    Return,
    Call,
    CallIndirect,
    Drop,
    Select,
    GetLocal,
    SetLocal,
    TeeLocal,
    GetGlobal,
    SetGlobal,
    I32Load,
    I64Load,
    F32Load,
    F64Load,
    I32Load8S,
    I32Load8U,
    I32Load16S,
    I32Load16U,
    I64Load8S,
    I64Load8U,
    I64Load16S,
    I64Load16U,
    I64Load32S,
    I64Load32U,
    I32Store,
    I64Store,
    F32Store,
    F64Store,
    I32Store8,
    I32Store16,
    I64Store8,
    I64Store16,
    I64Store32,
    CurrentMemory,
    GrowMemory,
    I32Const,
    I64Const,
    F32Const,
    F64Const,
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    I32WrapI64,
    I32TruncSF32,
    I32TruncUF32,
    I32TruncSF64,
    I32TruncUF64,
    I64ExtendSI32,
    I64ExtendUI32,
    I64TruncSF32,
    I64TruncUF32,
    I64TruncSF64,
    I64TruncUF64,
    F32ConvertSI32,
    F32ConvertUI32,
    F32ConvertSI64,
    F32ConvertUI64,
    F32DemoteF64,
    F64ConvertSI32,
    F64ConvertUI32,
    F64ConvertSI64,
    F64ConvertUI64,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
}

impl From<u8> for Opcode {
    fn from(x: u8) -> Self {
        use Opcode::*;

        match x {
            0x01 => Nop,
            0x02 => Block,
            0x03 => Loop,
            0x04 => If,
            0x05 => Else,
            0x0B => End,
            0x0C => Br,
            0x0D => BrIf,
            0x0E => BrTable,
            0x0F => Return,
            0x10 => Call,
            0x11 => CallIndirect,
            0x1A => Drop,
            0x1B => Select,
            0x20 => GetLocal,
            0x21 => SetLocal,
            0x22 => TeeLocal,
            0x23 => GetGlobal,
            0x24 => SetGlobal,
            0x28 => I32Load,
            0x29 => I64Load,
            0x2A => F32Load,
            0x2B => F64Load,
            0x2C => I32Load8S,
            0x2D => I32Load8U,
            0x2E => I32Load16S,
            0x2F => I32Load16U,
            0x30 => I64Load8S,
            0x31 => I64Load8U,
            0x32 => I64Load16S,
            0x33 => I64Load16U,
            0x34 => I64Load32S,
            0x35 => I64Load32U,
            0x36 => I32Store,
            0x37 => I64Store,
            0x38 => F32Store,
            0x39 => F64Store,
            0x3A => I32Store8,
            0x3B => I32Store16,
            0x3C => I64Store8,
            0x3D => I64Store16,
            0x3E => I64Store32,
            0x3F => CurrentMemory,
            0x40 => GrowMemory,
            0x41 => I32Const,
            0x42 => I64Const,
            0x43 => F32Const,
            0x44 => F64Const,
            0x45 => I32Eqz,
            0x46 => I32Eq,
            0x47 => I32Ne,
            0x48 => I32LtS,
            0x49 => I32LtU,
            0x4A => I32GtS,
            0x4B => I32GtU,
            0x4C => I32LeS,
            0x4D => I32LeU,
            0x4E => I32GeS,
            0x4F => I32GeU,
            0x50 => I64Eqz,
            0x51 => I64Eq,
            0x52 => I64Ne,
            0x53 => I64LtS,
            0x54 => I64LtU,
            0x55 => I64GtS,
            0x56 => I64GtU,
            0x57 => I64LeS,
            0x58 => I64LeU,
            0x59 => I64GeS,
            0x5A => I64GeU,
            0x5B => F32Eq,
            0x5C => F32Ne,
            0x5D => F32Lt,
            0x5E => F32Gt,
            0x5F => F32Le,
            0x60 => F32Ge,
            0x61 => F64Eq,
            0x62 => F64Ne,
            0x63 => F64Lt,
            0x64 => F64Gt,
            0x65 => F64Le,
            0x66 => F64Ge,
            0x67 => I32Clz,
            0x68 => I32Ctz,
            0x69 => I32Popcnt,
            0x6A => I32Add,
            0x6B => I32Sub,
            0x6C => I32Mul,
            0x6D => I32DivS,
            0x6E => I32DivU,
            0x6F => I32RemS,
            0x70 => I32RemU,
            0x71 => I32And,
            0x72 => I32Or,
            0x73 => I32Xor,
            0x74 => I32Shl,
            0x75 => I32ShrS,
            0x76 => I32ShrU,
            0x77 => I32Rotl,
            0x78 => I32Rotr,
            0x79 => I64Clz,
            0x7A => I64Ctz,
            0x7B => I64Popcnt,
            0x7C => I64Add,
            0x7D => I64Sub,
            0x7E => I64Mul,
            0x7F => I64DivS,
            0x80 => I64DivU,
            0x81 => I64RemS,
            0x82 => I64RemU,
            0x83 => I64And,
            0x84 => I64Or,
            0x85 => I64Xor,
            0x86 => I64Shl,
            0x87 => I64ShrS,
            0x88 => I64ShrU,
            0x89 => I64Rotl,
            0x8A => I64Rotr,
            0x8B => F32Abs,
            0x8C => F32Neg,
            0x8D => F32Ceil,
            0x8E => F32Floor,
            0x8F => F32Trunc,
            0x90 => F32Nearest,
            0x91 => F32Sqrt,
            0x92 => F32Add,
            0x93 => F32Sub,
            0x94 => F32Mul,
            0x95 => F32Div,
            0x96 => F32Min,
            0x97 => F32Max,
            0x98 => F32Copysign,
            0x99 => F64Abs,
            0x9A => F64Neg,
            0x9B => F64Ceil,
            0x9C => F64Floor,
            0x9D => F64Trunc,
            0x9E => F64Nearest,
            0x9F => F64Sqrt,
            0xA0 => F64Add,
            0xA1 => F64Sub,
            0xA2 => F64Mul,
            0xA3 => F64Div,
            0xA4 => F64Min,
            0xA5 => F64Max,
            0xA6 => F64Copysign,
            0xA7 => I32WrapI64,
            0xA8 => I32TruncSF32,
            0xA9 => I32TruncUF32,
            0xAA => I32TruncSF64,
            0xAB => I32TruncUF64,
            0xAC => I64ExtendSI32,
            0xAD => I64ExtendUI32,
            0xAE => I64TruncSF32,
            0xAF => I64TruncUF32,
            0xB0 => I64TruncSF64,
            0xB1 => I64TruncUF64,
            0xB2 => F32ConvertSI32,
            0xB3 => F32ConvertUI32,
            0xB4 => F32ConvertSI64,
            0xB5 => F32ConvertUI64,
            0xB6 => F32DemoteF64,
            0xB7 => F64ConvertSI32,
            0xB8 => F64ConvertUI32,
            0xB9 => F64ConvertSI64,
            0xBA => F64ConvertUI64,
            0xBB => F64PromoteF32,
            0xBC => I32ReinterpretF32,
            0xBD => I64ReinterpretF64,
            0xBE => F32ReinterpretI32,
            0xBF => F64ReinterpretI64,
            _ => Unexpected,
        }
    }
}
