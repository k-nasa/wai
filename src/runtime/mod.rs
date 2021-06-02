pub mod error;

use crate::instruction::Instruction;
use crate::types::RuntimeValue;
use crate::types::*;
use error::RuntimeError;
use std::error::Error;
use std::fmt::{self, Display};

type ValueStack = Vec<RuntimeValue>;

pub struct Runtime {
    pc: usize,
    instructions: Vec<Instruction>,

    value_stack: ValueStack,
}

impl Runtime {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            pc: 0,
            value_stack: Vec::new(),
        }
    }

    pub fn step() -> Result<(), RuntimeError> {
        todo!()
    }

    pub fn execute(&mut self, args: &[RuntimeValue]) -> Result<ValueStack, RuntimeError> {
        let mut locals = args.clone().to_vec();

        // let mut label_stack: Vec<u8> = Vec::new();
        // let mut activation_stack: Vec<u8> = Vec::new();
        let mut skip_else_or_end = false;

        let instructions = self.instructions.clone();

        for instruction in instructions {
            // TODO flagじゃなくてlabelでいい感じにしたい
            if skip_else_or_end {
                // TODO support else

                if instruction == Instruction::Else || instruction == Instruction::End {
                    skip_else_or_end = false;
                }

                continue;
            }
            match instruction {
                Instruction::Nop => {}
                Instruction::GetLocal(_) => self.value_stack.push(locals.pop().unwrap()),
                Instruction::SetLocal(i) => {
                    let v = self.value_stack.pop().unwrap();
                    locals.insert(usize::from(i), v);
                }
                Instruction::I32Add => {
                    let a = self.value_stack.pop().unwrap();
                    let b = self.value_stack.pop().unwrap();

                    self.value_stack
                        .push(RuntimeValue::I32(i32::from(a) + i32::from(b)));
                }
                Instruction::If(_block_type) => {
                    if self.value_stack.is_empty() {
                        return Err(RuntimeError::Custom(
                            "value stack is empty, if is expected value".to_string(),
                        ));
                    }

                    let condition = bool::from(self.value_stack.pop().unwrap());
                    dbg!(condition);
                    if condition {
                    } else {
                        skip_else_or_end = true;
                    }
                }
                Instruction::End => {}
                Instruction::I32Const(i) => self.value_stack.push(RuntimeValue::I32(i)),
                Instruction::Unexpected(op) => {
                    return Err(RuntimeError::Custom(format!(
                        "unexpected opcode: {:0x}",
                        op
                    )))
                }
                _ => {}
            }
        }
        Ok(self.value_stack.clone())
    }
}
