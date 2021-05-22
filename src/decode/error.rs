use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum DecodeError {
    InvalidWasmFile,
    InvalidNumeric,
    Unexpected,
    IOError(std::io::Error),
}

impl Error for DecodeError {}
impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::DecodeError::*;
        match self {
            InvalidWasmFile => write!(f, "invalid input file. not wasm file"),
            InvalidNumeric => write!(f, "invalid numeric"),
            Unexpected => write!(f, "unexpected byte(todo implement more information"),
            IOError(i) => write!(f, "io error: {}", i),
        }
    }
}

impl From<std::io::Error> for DecodeError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}
