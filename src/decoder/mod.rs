use crate::module::{Module, Section, SectionType};
use std::error::Error;
use std::fmt::{self, Display};
use std::io::Cursor;
use std::io::Read;

const MAGIC_NUMBER: &[u8] = b"\0asm";

#[derive(Debug)]
pub enum DecodeError {
    InvalidWasmFile,
    IOError(std::io::Error),
}

impl Error for DecodeError {}
impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::DecodeError::*;
        match self {
            InvalidWasmFile => write!(f, "invalid input file. not wasm file"),
            IOError(i) => write!(f, "io error: {}", i),
        }
    }
}

impl From<std::io::Error> for DecodeError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}

struct Decoder<'a> {
    reader: Cursor<&'a [u8]>,
}

/// wasmバイナリをデコードしてwasm moduleを返す
pub fn decode(buf: &[u8]) -> Result<Module, DecodeError> {
    let cursor = Cursor::new(buf);
    let mut decoder = Decoder::new(cursor);

    let mut m = Module::default();

    decoder.validate_wasm_format()?;
    m.version = decoder.decode_version()?;

    loop {
        if decoder.is_end() {
            break;
        }

        let (section_type, section_size) = decoder.decode_section_type()?;

        let section = match section_type {
            SectionType::Custom => decoder.decode_custom_section(section_size)?,
            SectionType::Type => decoder.decode_type_section(section_size)?,
            SectionType::Import => decoder.decode_import_section(section_size)?,
            SectionType::Function => decoder.decode_function_section(section_size)?,
            SectionType::Table => decoder.decode_table_section(section_size)?,
            SectionType::Memory => decoder.decode_memory_section(section_size)?,
            SectionType::Global => decoder.decode_global_section(section_size)?,
            SectionType::Export => decoder.decode_export_section(section_size)?,
            SectionType::Start => decoder.decode_start_section(section_size)?,
            SectionType::Element => decoder.decode_element_section(section_size)?,
            SectionType::Code => decoder.decode_code_section(section_size)?,
            SectionType::Data => decoder.decode_data_section(section_size)?,
            SectionType::Unsuport => todo!(),
        };

        m.take_in(section);
    }

    Ok(m)
}

impl<'a> Decoder<'a> {
    fn new(reader: Cursor<&'a [u8]>) -> Self {
        Self { reader }
    }

    fn is_end(&self) -> bool {
        self.reader.position() == self.reader.get_ref().len() as u64
    }

    /// wasmバイナリのマジックナンバーを見て適切なファイルか(本当にwasmか)をチェックする
    fn validate_wasm_format(&mut self) -> Result<(), DecodeError> {
        let mut magic_number = [0; 4];
        self.reader.read_exact(&mut magic_number)?;

        if magic_number != *MAGIC_NUMBER {
            return Err(DecodeError::InvalidWasmFile);
        }

        Ok(())
    }

    fn decode_version(&mut self) -> Result<u32, DecodeError> {
        let mut version = [0; 4];
        self.reader.read_exact(&mut version)?;

        Ok(u32::from_le_bytes(version))
    }

    fn decode_section_type(&mut self) -> Result<(SectionType, u8), DecodeError> {
        let mut section_number = [0; 1];
        self.reader.read_exact(&mut section_number)?;

        let mut section_size = [0; 1];
        self.reader.read_exact(&mut section_size)?;
        let section_size = section_size[0];

        let section_type = SectionType::from(section_number[0]);

        Ok((section_type, section_size))
    }

    fn decode_custom_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Custom(()))
    }

    fn decode_type_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Type(()))
    }

    fn decode_import_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Import(()))
    }

    fn decode_function_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Function(()))
    }

    fn decode_table_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Table(()))
    }

    fn decode_memory_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Memory(()))
    }

    fn decode_global_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Global(()))
    }

    fn decode_export_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Export(()))
    }

    fn decode_start_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Start(()))
    }

    fn decode_element_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Element(()))
    }

    fn decode_code_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Code(()))
    }

    fn decode_data_section(&mut self, size: u8) -> Result<Section, DecodeError> {
        let mut custom_section = vec![0; size as usize];
        self.reader.read_exact(&mut custom_section)?;

        Ok(Section::Data(()))
    }
}
