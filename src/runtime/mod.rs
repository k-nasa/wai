pub mod error;

use crate::instruction::Instruction;
use crate::types::RuntimeValue;
use error::RuntimeError;

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

    fn _step() -> Result<(), RuntimeError> {
        todo!()
    }

    pub fn execute(&mut self, args: &[RuntimeValue]) -> Result<ValueStack, RuntimeError> {
        let mut locals = Vec::from(args);

        // let mut label_stack: Vec<u8> = Vec::new();
        // let mut activation_stack: Vec<u8> = Vec::new();
        let mut skip_else_or_end = false;

        let instructions = self.instructions.clone();

        while let Some(&instruction) = instructions.get(self.pc) {
            self.pc += 1;

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
                Instruction::Block(BlockType) => todo!(),
                Instruction::Loop(BlockType) => todo!(),
                Instruction::If(BlockType) => todo!(),
                Instruction::If(_block_type) => {
                    if self.value_stack.is_empty() {
                        return Err(RuntimeError::Custom(
                            "value stack is empty, if is expected value".to_string(),
                        ));
                    }

                    let condition = bool::from(self.value_stack.pop().unwrap());
                    if condition {
                    } else {
                        skip_else_or_end = true;
                    }
                }
                Instruction::Else => todo!(),
                Instruction::End => {}
                Instruction::Br(_) => todo!(),
                Instruction::BrIf(_) => todo!(),
                Instruction::BrTable => todo!(),
                Instruction::Return => todo!(),
                Instruction::Call(_) => todo!(),
                Instruction::CallIndirect => todo!(),
                Instruction::Drop => todo!(),
                Instruction::Select => todo!(),
                Instruction::GetLocal(_) => self.value_stack.push(locals.pop().unwrap()),
                Instruction::SetLocal(i) => {
                    let v = self.value_stack.pop().unwrap();
                    locals.insert(usize::from(i), v);
                }
                Instruction::TeeLocal(_) => todo!(),
                Instruction::GetGlobal(_) => todo!(),
                Instruction::SetGlobal(_) => todo!(),
                Instruction::I32Load(_, _) => todo!(),
                Instruction::I64Load(_, _) => todo!(),
                Instruction::F32Load(_, _) => todo!(),
                Instruction::F64Load(_, _) => todo!(),
                Instruction::I32Load8S(_, _) => todo!(),
                Instruction::I32Load8U(_, _) => todo!(),
                Instruction::I32Load16S(_, _) => todo!(),
                Instruction::I32Load16U(_, _) => todo!(),
                Instruction::I64Load8S(_, _) => todo!(),
                Instruction::I64Load8U(_, _) => todo!(),
                Instruction::I64Load16S(_, _) => todo!(),
                Instruction::I64Load16U(_, _) => todo!(),
                Instruction::I64Load32S(_, _) => todo!(),
                Instruction::I64Load32U(_, _) => todo!(),
                Instruction::I32Store(_, _) => todo!(),
                Instruction::I64Store(_, _) => todo!(),
                Instruction::F32Store(_, _) => todo!(),
                Instruction::F64Store(_, _) => todo!(),
                Instruction::I32Store8(_, _) => todo!(),
                Instruction::I32Store16(_, _) => todo!(),
                Instruction::I64Store8(_, _) => todo!(),
                Instruction::I64Store16(_, _) => todo!(),
                Instruction::I64Store32(_, _) => todo!(),
                Instruction::CurrentMemory(VerUintN) => todo!(),
                Instruction::GrowMemory(VerUintN) => todo!(),
                Instruction::I32Const(v) => self.value_stack.push(RuntimeValue::I32(v)),
                Instruction::I64Const(v) => self.value_stack.push(RuntimeValue::I64(v)),
                Instruction::F32Const(v) => self.value_stack.push(RuntimeValue::F32(v)),
                Instruction::F64Const(v) => self.value_stack.push(RuntimeValue::F64(v)),
                Instruction::I32Add => {
                    let a = self.value_stack.pop().unwrap();
                    let b = self.value_stack.pop().unwrap();

                    self.value_stack
                        .push(RuntimeValue::I32(i32::from(a) + i32::from(b)));
                }
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
