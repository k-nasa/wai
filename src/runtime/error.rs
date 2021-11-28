use crate::types::*;
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum RuntimeError {
    NotFound(String),
    ExpectCodeSection,
    ExpectValueStack,
    ExpectLabelStack,
    ExpectActivationStack,
    Unimplemented,
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
            Unimplemented => write!(f, "unimplemented"),
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

impl PartialEq for RuntimeError {
    // TODO implement
    fn eq(&self, other: &Self) -> bool {
        if self == &RuntimeError::Unimplemented && other == &RuntimeError::Unimplemented {
            return true;
        }
        return false;
    }
}
