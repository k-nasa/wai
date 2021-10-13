use core::fmt::{self, Display};
use std::error::Error;

use alloc::string::String;

#[derive(Debug)]
pub enum DecodeError {
    InvalidWasmFile,
    InvalidNumeric(String),
    Unexpected(String),
    IOError(std::io::Error),
}

impl Error for DecodeError {}
impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::DecodeError::*;
        match self {
            InvalidWasmFile => write!(f, "invalid input file. not wasm file"),
            InvalidNumeric(s) => write!(f, "invalid numeric: {}", s),
            Unexpected(s) => write!(f, "unexpected byte. details: {}", s),
            IOError(i) => write!(f, "io error: {}", i),
        }
    }
}

impl From<std::io::Error> for DecodeError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}
