use crate::types::*;
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum RuntimeError {
    NotFound(String),
    ExpectCodeSection,
    InvalidArgs(Vec<ValueType>, Vec<ValueType>),
    IOError(std::io::Error),
    Custom(String),
}

impl Error for RuntimeError {}
impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RuntimeError::*;
        match self {
            NotFound(name) => write!(f, "'{}'' is not found", name),
            InvalidArgs(expect, actual) => write!(
                f,
                "Invalid argument: expect {:?},but got {:?}",
                expect, actual
            ),
            ExpectCodeSection => {
                write!(f, "not found code section. wasmi is expected code section")
            }
            IOError(i) => write!(f, "io error: {}", i),
            Custom(s) => write!(f, "{}", s),
        }
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}
