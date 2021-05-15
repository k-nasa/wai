use crate::module::Module;
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum DecodeError {
    InvalidWasmFile,
}

impl Error for DecodeError {}
impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::DecodeError::*;
        match self {
            InvalidWasmFile => write!(f, "invalid input file. not wasm file"),
        }
    }
}

pub fn decode(buf: &[u8]) -> Result<Module, DecodeError> {
    Ok(Module {})
}
