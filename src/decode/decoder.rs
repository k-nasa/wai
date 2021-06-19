use crate::decode::error::DecodeError;
use crate::instruction::Instruction;
use crate::module::{Section, SectionType};
use crate::opcode::Opcode;
use crate::types::*;
use std::convert::TryFrom;
use std::io::Cursor;
use std::io::Read;

const MAGIC_NUMBER: &[u8] = b"\0asm";
const FUNC_TYPE: u8 = 0x60;

pub(crate) struct Decoder<'a> {
    reader: Cursor<&'a [u8]>,
}

impl<'a> Decoder<'a> {
    pub(crate) fn new(reader: Cursor<&'a [u8]>) -> Self {
        Self { reader }
    }

    pub(crate) fn is_end(&self) -> bool {
        self.reader.position() == self.reader.get_ref().len() as u64
    }

    /// wasmバイナリのマジックナンバーを見て適切なファイルか(本当にwasmか)をチェックする
    pub(crate) fn validate_wasm_format(&mut self) -> Result<(), DecodeError> {
        let mut magic_number = [0; 4];
        self.reader.read_exact(&mut magic_number)?;

        if magic_number != *MAGIC_NUMBER {
            return Err(DecodeError::InvalidWasmFile);
        }

        Ok(())
    }

    pub(crate) fn decode_version(&mut self) -> Result<u32, DecodeError> {
        let mut version = [0; 4];
        self.reader.read_exact(&mut version)?;

        Ok(u32::from_le_bytes(version))
    }

    pub(crate) fn decode_section_type(&mut self) -> Result<(SectionType, u32), DecodeError> {
        let mut section_number = [0; 1];
        self.reader.read_exact(&mut section_number)?;

        let section_size = self.decode_ver_uint_n()?;
        let section_type = SectionType::from(section_number[0]);

        Ok((section_type, section_size.into()))
    }

    pub(crate) fn decode_section(
        &mut self,
        section_type: SectionType,
        section_size: u32,
    ) -> Result<Section, DecodeError> {
        let section = match section_type {
            SectionType::Custom => self.decode_custom_section(section_size)?,
            SectionType::Type => self.decode_type_section(section_size)?,
            SectionType::Import => self.decode_import_section(section_size)?,
            SectionType::Function => self.decode_function_section(section_size)?,
            SectionType::Table => self.decode_table_section(section_size)?,
            SectionType::Memory => self.decode_memory_section(section_size)?,
            SectionType::Global => self.decode_global_section(section_size)?,
            SectionType::Export => self.decode_export_section(section_size)?,
            SectionType::Start => self.decode_start_section(section_size)?,
            SectionType::Element => self.decode_element_section(section_size)?,
            SectionType::Code => self.decode_code_section(section_size)?,
            SectionType::Data => self.decode_data_section(section_size)?,
            SectionType::Unsuport => todo!(),
        };

        Ok(section)
    }

    fn decode_custom_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Custom(())) // TODO implement!
    }

    fn decode_type_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut type_section = vec![0; size as usize];
        self.reader.read_exact(&mut type_section)?;

        let mut type_section_decoder = Decoder::new(Cursor::new(&type_section));
        let mut type_section = TypeSection {
            entries: Vec::new(),
        };

        let entry_count = type_section_decoder.decode_ver_uint_n()?;
        for _ in 0..entry_count.into() {
            let func_type = type_section_decoder.read_next()?;
            if func_type != FUNC_TYPE {
                return Err(DecodeError::Unexpected(String::new()));
            }

            let mut func_type = FuncType {
                params: vec![],
                returns: vec![],
            };

            let arg_count = type_section_decoder.decode_ver_uint_n()?;
            for _ in 0..arg_count.into() {
                let t = type_section_decoder.decode_ver_uint_n()?;
                func_type.params.push(ValueType::from(t));
            }

            let returns_count = type_section_decoder.decode_ver_uint_n()?;
            for _ in 0..returns_count.into() {
                let t = type_section_decoder.decode_ver_uint_n()?;
                func_type.returns.push(ValueType::from(t));
            }

            type_section.entries.push(func_type)
        }

        Ok(Section::Type(type_section))
    }

    fn decode_import_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Import(()))
    }

    fn decode_function_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut func_section = vec![0; size as usize];
        self.reader.read_exact(&mut func_section)?;

        let mut func_section_decoder = Decoder::new(Cursor::new(&func_section));
        let mut func_section = FunctionSection { types: Vec::new() };

        let count: u32 = func_section_decoder.decode_ver_uint_n()?.into();
        for _ in 0..count {
            let t = func_section_decoder.decode_ver_uint_n()?;
            func_section.types.push(t.into());
        }

        Ok(Section::Function(func_section))
    }

    fn decode_table_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Table(()))
    }

    fn decode_memory_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Memory(()))
    }

    fn decode_global_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Global(()))
    }

    fn decode_export_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut section = vec![0; size as usize];
        self.reader.read_exact(&mut section)?;

        let mut export_section_decoder = Decoder::new(Cursor::new(&section));
        let mut export_section = ExportSection {
            entries: Vec::new(),
        };

        let count: u32 = export_section_decoder.decode_ver_uint_n()?.into();
        for _ in 0..count {
            let field_len = export_section_decoder.decode_ver_uint_n()?;
            let field_str =
                String::from_utf8(export_section_decoder.read_byte(field_len.into())?).unwrap();
            let kind = ExternalKind::from(export_section_decoder.decode_ver_uint_n()?);
            let index = export_section_decoder.decode_ver_uint_n()?.into();

            let entry = ExportEntry {
                field_str,
                kind,
                index,
            };
            export_section.entries.push(entry);
        }

        Ok(Section::Export(export_section))
    }

    fn decode_start_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Start(()))
    }

    fn decode_element_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Element(()))
    }

    fn decode_code_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut section = vec![0; size as usize];
        self.reader.read_exact(&mut section)?;

        let mut code_section_decoder = Decoder::new(Cursor::new(&section));
        let mut code_section = CodeSection { bodies: Vec::new() };

        let count: u32 = code_section_decoder.decode_ver_uint_n()?.into();

        for _ in 0..count {
            let body_size = code_section_decoder.decode_ver_uint_n()?;
            let body_bytes = &code_section_decoder.read_byte(body_size.into())?;
            let mut body = Decoder::new(Cursor::new(body_bytes));

            let local_count = body.decode_ver_uint_n()?;

            let mut function_body = FunctionBody {
                locales: Vec::new(),
                code: Vec::new(),
            };

            for _ in 0..local_count.into() {
                let count = body.decode_ver_uint_n()?;
                let t = body.decode_ver_uint_n()?;
                let local_entry = LocalEntry {
                    count: count.into(),
                    value_type: ValueType::from(t),
                };

                function_body.locales.push(local_entry);
            }

            function_body.code = body.decode_function_body()?;
            code_section.bodies.push(function_body);
        }

        Ok(Section::Code(code_section))
    }

    fn decode_data_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Data(()))
    }

    fn decode_ver_uint_n(&mut self) -> Result<VerUintN, DecodeError> {
        let mut value = 0;
        let mut i = 0;
        loop {
            let bytes = u32::from(self.read_next()?);
            value += (bytes & 0x7f)
                .checked_shl(i * 7)
                .ok_or(DecodeError::InvalidNumeric)?;

            i += 1;

            if bytes & 0x80 == 0 {
                break;
            }
        }
        Ok(VerUintN::from(value))
    }

    fn decode_function_body(&mut self) -> Result<Vec<Instruction>, DecodeError> {
        let mut instructions = Vec::new();
        loop {
            let opcode = Opcode::try_from(self.read_next()?)?;
            if self.is_end() {
                break;
            }

            let instruction = match opcode {
                // expect BlockType
                Opcode::Block => Instruction::Block(BlockType::from(self.read_next()?)),
                Opcode::Loop => Instruction::Loop(BlockType::from(self.read_next()?)),
                Opcode::If => Instruction::If(BlockType::from(self.read_next()?)),

                // expect VerUintN
                Opcode::Br => Instruction::Br(self.decode_ver_uint_n()?),
                Opcode::BrIf => Instruction::BrIf(self.decode_ver_uint_n()?),
                Opcode::GetLocal => Instruction::GetLocal(self.decode_ver_uint_n()?),
                Opcode::SetLocal => Instruction::SetLocal(self.decode_ver_uint_n()?),
                Opcode::TeeLocal => Instruction::TeeLocal(self.decode_ver_uint_n()?),
                Opcode::GetGlobal => Instruction::GetGlobal(self.decode_ver_uint_n()?),
                Opcode::SetGlobal => Instruction::SetGlobal(self.decode_ver_uint_n()?),
                Opcode::Call => Instruction::Call(self.decode_ver_uint_n()?),
                Opcode::CurrentMemory => Instruction::CurrentMemory(self.decode_ver_uint_n()?),
                Opcode::GrowMemory => Instruction::GrowMemory(self.decode_ver_uint_n()?),

                Opcode::BrTable => {
                    let target_count = self.decode_ver_uint_n()?;
                    let mut target_tables = vec![];
                    for _ in 0..u32::from(target_count) {
                        target_tables.push(self.decode_ver_uint_n()?);
                    }
                    let default_target = self.decode_ver_uint_n()?;

                    Instruction::BrTable(target_tables, default_target)
                }
                Opcode::CallIndirect => todo!(),

                Opcode::I32Load => Instruction::I32Load(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I64Load => Instruction::I64Load(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::F32Load => Instruction::F32Load(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::F64Load => Instruction::F64Load(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I32Load8S => Instruction::I32Load8S(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I32Load8U => Instruction::I32Load8U(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I32Load16S => Instruction::I32Load16S(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I32Load16U => Instruction::I32Load16U(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I64Load8S => Instruction::I64Load8S(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I64Load8U => Instruction::I64Load8U(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I64Load16S => Instruction::I64Load16S(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I64Load16U => Instruction::I64Load16U(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I64Load32S => Instruction::I64Load32S(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I64Load32U => Instruction::I64Load32U(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I32Store => Instruction::I32Store(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I64Store => Instruction::I64Store(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::F32Store => Instruction::F32Store(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::F64Store => Instruction::F64Store(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I32Store8 => Instruction::I32Store8(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I32Store16 => Instruction::I32Store16(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I64Store8 => Instruction::I64Store8(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I64Store16 => Instruction::I64Store16(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),
                Opcode::I64Store32 => Instruction::I64Store32(
                    u32::from(self.decode_ver_uint_n()?),
                    u32::from(self.decode_ver_uint_n()?),
                ),

                Opcode::I32Const => Instruction::I32Const(i32::from(self.decode_ver_uint_n()?)),
                Opcode::I64Const => Instruction::I64Const(i64::from(self.decode_ver_uint_n()?)),
                Opcode::F32Const => Instruction::F32Const(f32::from(self.decode_ver_uint_n()?)),
                Opcode::F64Const => Instruction::F64Const(f64::from(self.decode_ver_uint_n()?)),
                _ => Instruction::from(opcode),
            };

            instructions.push(instruction);
        }

        Ok(instructions)
    }

    fn read_next(&mut self) -> Result<u8, DecodeError> {
        let mut buf = [0; 1];
        self.reader.read_exact(&mut buf)?;

        Ok(buf[0])
    }

    fn read_byte(&mut self, size: usize) -> Result<Vec<u8>, DecodeError> {
        let mut buf = vec![0; size];
        self.reader.read_exact(&mut buf)?;

        Ok(buf)
    }

    // fn read_to_end(&mut self) -> Result<Vec<u8>, DecodeError> {
    //     let mut buf = vec![];
    //     self.reader.read_to_end(&mut buf)?;
    //
    //     Ok(buf)
    // }
}
