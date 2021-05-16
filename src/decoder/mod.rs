use crate::module::{Module, SectionType};
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

/// wasmバイナリをデコードしてwasm moduleを返す
pub fn decode(buf: &[u8]) -> Result<Module, DecodeError> {
    let mut cursor = Cursor::new(buf);
    let mut m = Module::default();

    validate_wasm_format(&mut cursor)?;
    m.version = decode_version(&mut cursor)?;

    loop {
        if cursor.position() >= buf.len() as u64 {
            break;
        }
        let (section_type, section_size) = decode_section_type(&mut cursor)?;
        println!("{:?}, {}", section_type, section_size);

        let section = match section_type {
            SectionType::Custom => decode_custom_section(section_size, &mut cursor)?,
            SectionType::Type => decode_custom_section(section_size, &mut cursor)?,
            SectionType::Import => decode_custom_section(section_size, &mut cursor)?,
            SectionType::Function => decode_custom_section(section_size, &mut cursor)?,
            SectionType::Table => decode_custom_section(section_size, &mut cursor)?,
            SectionType::Memory => decode_custom_section(section_size, &mut cursor)?,
            SectionType::Global => decode_custom_section(section_size, &mut cursor)?,
            SectionType::Export => decode_custom_section(section_size, &mut cursor)?,
            SectionType::Start => decode_custom_section(section_size, &mut cursor)?,
            SectionType::Element => decode_custom_section(section_size, &mut cursor)?,
            SectionType::Code => decode_custom_section(section_size, &mut cursor)?,
            SectionType::Data => decode_custom_section(section_size, &mut cursor)?,
            SectionType::Unsuport => (),
        };

        m.take_in(section_type, section);
    }

    Ok(m)
}

/// wasmバイナリのマジックナンバーを見て適切なファイルか(本当にwasmか)をチェックする
fn validate_wasm_format(mut reader: impl Read) -> Result<(), DecodeError> {
    let mut magic_number = [0; 4];
    reader.read_exact(&mut magic_number)?;

    if &magic_number != MAGIC_NUMBER {
        return Err(DecodeError::InvalidWasmFile);
    }

    Ok(())
}

fn decode_version(mut reader: impl Read) -> Result<u32, DecodeError> {
    let mut version = [0; 4];
    reader.read_exact(&mut version)?;

    Ok(u32::from_le_bytes(version))
}

fn decode_section_type(mut reader: impl Read) -> Result<(SectionType, u8), DecodeError> {
    let mut section_number = [0; 1];
    reader.read_exact(&mut section_number)?;

    let mut section_size = [0; 1];
    reader.read_exact(&mut section_size)?;
    let section_size = section_size[0];

    let section_type = SectionType::from(section_number[0]);

    Ok((section_type, section_size))
}

fn decode_custom_section(size: u8, mut reader: impl Read) -> Result<(), DecodeError> {
    let mut custom_section = vec![0; size as usize];
    reader.read_exact(&mut custom_section)?;

    Ok(())
}
