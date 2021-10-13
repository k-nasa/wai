use alloc::string::String;
use alloc::vec::Vec;

use crate::types::*;
use core::fmt::{self, Display};
use std::error::Error;

#[derive(Debug)]
pub enum RuntimeError {
    NotFound(String),
    ExpectCodeSection,
    ExpectValueStack,
    ExpectLabelStack,
    ExpectActivationStack,
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
            ExpectValueStack => write!(f, "expect value stack, but nothing"),
            ExpectActivationStack => write!(f, "expect activation stack, but nothing"),
            ExpectLabelStack => write!(f, "expect label stack, but nothing"),
            InvalidArgs(expect, actual) => write!(
                f,
                "Invalid argument: expect {:?},but got {:?}",
                expect, actual
            ),
            ExpectCodeSection => {
                write!(f, "not found code section. wai is expected code section")
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
