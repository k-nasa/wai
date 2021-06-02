pub mod error;

use crate::instruction::Instruction;
use crate::types::RuntimeValue;
use crate::types::*;
use error::RuntimeError;
use std::error::Error;
use std::fmt::{self, Display};

pub struct Runtime {
    pc: usize,
    instructions: Vec<Instruction>,

    value_stack: Vec<RuntimeValue>,
}

impl Runtime {
    fn step() -> Result<(), RuntimeError> {
        todo!()
    }
}
