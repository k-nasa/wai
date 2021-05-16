use crate::decoder::error::DecodeError;
use crate::module::{Section, SectionType};
use std::io::Cursor;
use std::io::Read;

const MAGIC_NUMBER: &[u8] = b"\0asm";

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

    pub(crate) fn decode_section_type(&mut self) -> Result<(SectionType, u8), DecodeError> {
        let mut section_number = [0; 1];
        self.reader.read_exact(&mut section_number)?;

        let mut section_size = [0; 1];
        self.reader.read_exact(&mut section_size)?;
        let section_size = section_size[0];

        let section_type = SectionType::from(section_number[0]);

        Ok((section_type, section_size))
    }

    pub(crate) fn decode_section(
        &mut self,
        section_type: SectionType,
        section_size: u8,
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

    pub(crate) fn decode_custom_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Custom(()))
    }

    pub(crate) fn decode_type_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Type(()))
    }

    pub(crate) fn decode_import_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Import(()))
    }

    pub(crate) fn decode_function_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Function(()))
    }

    pub(crate) fn decode_table_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Table(()))
    }

    pub(crate) fn decode_memory_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Memory(()))
    }

    pub(crate) fn decode_global_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Global(()))
    }

    pub(crate) fn decode_export_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Export(()))
    }

    pub(crate) fn decode_start_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Start(()))
    }

    pub(crate) fn decode_element_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Element(()))
    }

    pub(crate) fn decode_code_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Code(()))
    }

    pub(crate) fn decode_data_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Data(()))
    }
}
