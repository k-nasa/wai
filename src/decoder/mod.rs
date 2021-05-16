use crate::module::Module;
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

    validate_wasm_format(&mut cursor)?;
    let version = decode_version(&mut cursor)?;

    Ok(Module { version })
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
    println!("{:?}", version);

    Ok(u32::from_le_bytes(version))
}
