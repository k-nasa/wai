use crate::decoder::error::DecodeError;
use crate::module::{Section, SectionType};
use crate::types::*;
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

    pub(crate) fn decode_custom_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Custom(())) // TODO implement!
    }

    pub(crate) fn decode_type_section(&mut self, size: u32) -> Result<Section, DecodeError> {
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
                return Err(DecodeError::Unexpected);
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

    pub(crate) fn decode_import_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Import(()))
    }

    pub(crate) fn decode_function_section(&mut self, size: u32) -> Result<Section, DecodeError> {
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

    pub(crate) fn decode_table_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Table(()))
    }

    pub(crate) fn decode_memory_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Memory(()))
    }

    pub(crate) fn decode_global_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Global(()))
    }

    pub(crate) fn decode_export_section(&mut self, size: u32) -> Result<Section, DecodeError> {
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

    pub(crate) fn decode_start_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Start(()))
    }

    pub(crate) fn decode_element_section(&mut self, size: u32) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Element(()))
    }

    pub(crate) fn decode_code_section(&mut self, size: u32) -> Result<Section, DecodeError> {
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

            function_body.code = body.read_to_end()?;

            code_section.bodies.push(function_body);
        }

        Ok(Section::Code(code_section))
    }

    pub(crate) fn decode_data_section(&mut self, size: u32) -> Result<Section, DecodeError> {
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

    fn read_next(&mut self) -> Result<u8, DecodeError> {
        let mut buf = [0; 1];
        self.reader.read(&mut buf)?;

        Ok(buf[0])
    }

    fn read_byte(&mut self, size: usize) -> Result<Vec<u8>, DecodeError> {
        let mut buf = vec![0; size];
        self.reader.read(&mut buf)?;

        Ok(buf)
    }

    fn read_to_end(&mut self) -> Result<Vec<u8>, DecodeError> {
        let mut buf = vec![];
        self.reader.read_to_end(&mut buf)?;

        Ok(buf)
    }
}
