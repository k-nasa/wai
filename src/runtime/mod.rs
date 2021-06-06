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
                Instruction::Block(_) => todo!(),
                Instruction::Loop(_) => todo!(),
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
                Instruction::CurrentMemory(_) => todo!(),
                Instruction::GrowMemory(_) => todo!(),
                Instruction::I32Const(v) => self.value_stack.push(RuntimeValue::I32(v)),
                Instruction::I64Const(v) => self.value_stack.push(RuntimeValue::I64(v)),
                Instruction::F32Const(v) => self.value_stack.push(RuntimeValue::F32(v)),
                Instruction::F64Const(v) => self.value_stack.push(RuntimeValue::F64(v)),
                Instruction::I32Eqz => {
                    let flag = if self.value_stack.pop().unwrap() == RuntimeValue::I32(0) {
                        1
                    } else {
                        0
                    };
                    self.value_stack.push(RuntimeValue::I32(flag));
                }
                Instruction::I32Eq => self.eq::<i32>(),
                Instruction::I32Ne => self.neq::<i32>(),
                Instruction::I32LtS => self.lt_s::<i32>(),
                Instruction::I32LtU => self.lt_u::<i32>(),
                Instruction::I32GtS => self.gt_s::<i32>(),
                Instruction::I32GtU => self.gt_s::<i32>(),
                Instruction::I32LeS => self.le_s::<i32>(),
                Instruction::I32LeU => self.le_u::<i32>(),
                Instruction::I32GeS => self.ge_s::<i32>(),
                Instruction::I32GeU => self.ge_u::<i32>(),

                Instruction::I64Eqz => {
                    let flag = if self.value_stack.pop().unwrap() == RuntimeValue::I64(0) {
                        1
                    } else {
                        0
                    };
                    self.value_stack.push(RuntimeValue::I64(flag));
                }
                Instruction::I64Eq => self.eq::<i64>(),
                Instruction::I64Ne => self.neq::<i64>(),
                Instruction::I64LtS => self.lt_s::<i64>(),
                Instruction::I64LtU => self.lt_u::<i64>(),
                Instruction::I64GtS => self.gt_s::<i64>(),
                Instruction::I64GtU => self.gt_u::<i64>(),
                Instruction::I64LeS => self.le_s::<i64>(),
                Instruction::I64LeU => self.le_u::<i64>(),
                Instruction::I64GeS => self.ge_s::<i64>(),
                Instruction::I64GeU => self.ge_u::<i64>(),

                Instruction::F32Eq => self.eq::<f32>(),
                Instruction::F32Ne => self.neq::<f32>(),
                Instruction::F32Lt => self.lt::<f32>(),
                Instruction::F32Gt => self.gt::<f32>(),
                Instruction::F32Le => self.le::<f32>(),
                Instruction::F32Ge => self.ge::<f32>(),

                Instruction::F64Eq => self.eq::<f64>(),
                Instruction::F64Ne => self.neq::<f64>(),
                Instruction::F64Lt => self.lt::<f64>(),
                Instruction::F64Gt => self.gt::<f64>(),
                Instruction::F64Le => self.le::<f64>(),
                Instruction::F64Ge => self.ge::<f64>(),

                Instruction::I32Clz => todo!(),
                Instruction::I32Ctz => todo!(),
                Instruction::I32Popcnt => todo!(),
                Instruction::I32Add => {
                    let (a, b) = self.pop_lr::<i32>();
                    self.value_stack.push(RuntimeValue::I32(a + b));
                }
                Instruction::I32Sub => {
                    let (a, b) = self.pop_lr::<i32>();
                    self.value_stack.push(RuntimeValue::I32(a - b));
                }
                Instruction::I32Mul => {
                    let (a, b) = self.pop_lr::<i32>();
                    self.value_stack.push(RuntimeValue::I32(a * b));
                }
                Instruction::I32DivS => {
                    // TODO test
                    let (a, b) = self.pop_lr::<i32>();
                    self.value_stack.push(RuntimeValue::I32(a / b));
                }
                Instruction::I32DivU => {
                    // TODO test
                    let (a, b) = self.pop_lr::<i32>();
                    self.value_stack
                        .push(RuntimeValue::I32(a / b as u32 as i32)); // NOTE 適当にunsignedにしている。これで良いのかよう分からん
                }
                Instruction::I32RemS => {
                    // TODO test
                    let (a, b) = self.pop_lr::<i32>();
                    self.value_stack.push(RuntimeValue::I32(a % b));
                }
                Instruction::I32RemU => {
                    // TODO test
                    let (a, b) = self.pop_lr::<i32>();
                    self.value_stack
                        .push(RuntimeValue::I32(a % b as u32 as i32));
                }
                Instruction::I32And => {
                    let (a, b) = self.pop_lr::<i32>();
                    self.value_stack.push(RuntimeValue::I32(a & b));
                }
                Instruction::I32Or => {
                    let (a, b) = self.pop_lr::<i32>();
                    self.value_stack.push(RuntimeValue::I32(a | b));
                }
                Instruction::I32Xor => {
                    let (a, b) = self.pop_lr::<i32>();
                    self.value_stack.push(RuntimeValue::I32(a ^ b));
                }
                Instruction::I32Shl => todo!(),
                Instruction::I32ShrS => todo!(),
                Instruction::I32ShrU => todo!(),
                Instruction::I32Rotl => todo!(),
                Instruction::I32Rotr => todo!(),
                Instruction::I64Clz => todo!(),
                Instruction::I64Ctz => todo!(),
                Instruction::I64Popcnt => todo!(),
                Instruction::I64Add => {
                    let (a, b) = self.pop_lr::<i64>();
                    self.value_stack.push(RuntimeValue::I64(a + b));
                }
                Instruction::I64Sub => {
                    let (a, b) = self.pop_lr::<i64>();
                    self.value_stack.push(RuntimeValue::I64(a - b));
                }
                Instruction::I64Mul => {
                    let (a, b) = self.pop_lr::<i64>();
                    self.value_stack.push(RuntimeValue::I64(a * b));
                }
                Instruction::I64DivS => {
                    let (a, b) = self.pop_lr::<i64>();
                    self.value_stack.push(RuntimeValue::I64(a / b));
                }
                Instruction::I64DivU => {
                    let (a, b) = self.pop_lr::<i64>();
                    self.value_stack
                        .push(RuntimeValue::I64(a / b as u64 as i64));
                }
                Instruction::I64RemS => {
                    let (a, b) = self.pop_lr::<i64>();
                    self.value_stack.push(RuntimeValue::I64(a % b));
                }
                Instruction::I64RemU => {
                    let (a, b) = self.pop_lr::<i64>();
                    self.value_stack
                        .push(RuntimeValue::I64(a % b as u64 as i64));
                }
                Instruction::I64And => {
                    let (a, b) = self.pop_lr::<i64>();
                    self.value_stack.push(RuntimeValue::I64(a & b));
                }
                Instruction::I64Or => {
                    let (a, b) = self.pop_lr::<i64>();
                    self.value_stack.push(RuntimeValue::I64(a | b));
                }
                Instruction::I64Xor => {
                    let (a, b) = self.pop_lr::<i64>();
                    self.value_stack.push(RuntimeValue::I64(a ^ b));
                }
                Instruction::I64Shl => todo!(),
                Instruction::I64ShrS => todo!(),
                Instruction::I64ShrU => todo!(),
                Instruction::I64Rotl => todo!(),
                Instruction::I64Rotr => todo!(),
                Instruction::F32Abs => todo!(),
                Instruction::F32Neg => todo!(),
                Instruction::F32Ceil => todo!(),
                Instruction::F32Floor => todo!(),
                Instruction::F32Trunc => todo!(),
                Instruction::F32Nearest => todo!(),
                Instruction::F32Sqrt => todo!(),
                Instruction::F32Add => todo!(),
                Instruction::F32Sub => todo!(),
                Instruction::F32Mul => todo!(),
                Instruction::F32Div => todo!(),
                Instruction::F32Min => todo!(),
                Instruction::F32Max => todo!(),
                Instruction::F32Copysign => todo!(),
                Instruction::F64Abs => todo!(),
                Instruction::F64Neg => todo!(),
                Instruction::F64Ceil => todo!(),
                Instruction::F64Floor => todo!(),
                Instruction::F64Trunc => todo!(),
                Instruction::F64Nearest => todo!(),
                Instruction::F64Sqrt => todo!(),
                Instruction::F64Add => todo!(),
                Instruction::F64Sub => todo!(),
                Instruction::F64Mul => todo!(),
                Instruction::F64Div => todo!(),
                Instruction::F64Min => todo!(),
                Instruction::F64Max => todo!(),
                Instruction::F64Copysign => todo!(),
                Instruction::I32WrapI64 => todo!(),
                Instruction::I32TruncSF32 => todo!(),
                Instruction::I32TruncUF32 => todo!(),
                Instruction::I32TruncSF64 => todo!(),
                Instruction::I32TruncUF64 => todo!(),
                Instruction::I64ExtendSI32 => todo!(),
                Instruction::I64ExtendUI32 => todo!(),
                Instruction::I64TruncSF32 => todo!(),
                Instruction::I64TruncUF32 => todo!(),
                Instruction::I64TruncSF64 => todo!(),
                Instruction::I64TruncUF64 => todo!(),
                Instruction::F32ConvertSI32 => todo!(),
                Instruction::F32ConvertUI32 => todo!(),
                Instruction::F32ConvertSI64 => todo!(),
                Instruction::F32ConvertUI64 => todo!(),
                Instruction::F32DemoteF64 => todo!(),
                Instruction::F64ConvertSI32 => todo!(),
                Instruction::F64ConvertUI32 => todo!(),
                Instruction::F64ConvertSI64 => todo!(),
                Instruction::F64ConvertUI64 => todo!(),
                Instruction::F64PromoteF32 => todo!(),
                Instruction::I32ReinterpretF32 => todo!(),
                Instruction::I64ReinterpretF64 => todo!(),
                Instruction::F32ReinterpretI32 => todo!(),
                Instruction::F64ReinterpretI64 => todo!(),
            }
        }
        Ok(self.value_stack.clone())
    }

    fn pop_lr<T>(&mut self) -> (T, T)
    where
        T: From<RuntimeValue>,
    {
        let l = self.value_stack.pop().unwrap();
        let r = self.value_stack.pop().unwrap();

        (T::from(l), T::from(r))
    }

    fn eq<T>(&mut self)
    where
        T: From<RuntimeValue> + PartialEq,
    {
        let (a, b) = self.pop_lr::<T>();
        let flag = if a == b { 1 } else { 0 };
        self.value_stack.push(RuntimeValue::I32(flag));
    }

    fn neq<T>(&mut self)
    where
        T: From<RuntimeValue> + PartialEq,
    {
        let (a, b) = self.pop_lr::<T>();
        let flag = if a != b { 1 } else { 0 };
        self.value_stack.push(RuntimeValue::I32(flag));
    }

    fn lt<T>(&mut self)
    where
        T: From<RuntimeValue> + PartialOrd,
    {
        let (a, b) = self.pop_lr::<T>();
        let flag = if a < b { 1 } else { 0 };
        self.value_stack.push(RuntimeValue::I32(flag));
    }

    fn gt<T>(&mut self)
    where
        T: From<RuntimeValue> + PartialOrd,
    {
        let (a, b) = self.pop_lr::<T>();
        let flag = if a > b { 1 } else { 0 };
        self.value_stack.push(RuntimeValue::I32(flag));
    }

    fn le<T>(&mut self)
    where
        T: From<RuntimeValue> + PartialOrd,
    {
        let (a, b) = self.pop_lr::<T>();
        let flag = if a <= b { 1 } else { 0 };
        self.value_stack.push(RuntimeValue::I32(flag));
    }

    fn ge<T>(&mut self)
    where
        T: From<RuntimeValue> + PartialOrd,
    {
        let (a, b) = self.pop_lr::<T>();
        let flag = if a >= b { 1 } else { 0 };
        self.value_stack.push(RuntimeValue::I32(flag));
    }

    fn lt_s<T>(&mut self)
    where
        T: From<RuntimeValue> + PartialOrd,
    {
        let (a, b) = self.pop_lr::<T>();
        let flag = if a < b { 1 } else { 0 };
        self.value_stack.push(RuntimeValue::I32(flag));
    }

    fn lt_u<T>(&mut self)
    where
        T: From<RuntimeValue> + PartialOrd,
    {
        let (a, b) = self.pop_lr::<T>();
        let flag = if a < b { 1 } else { 0 };
        // TODO unsigned
        self.value_stack.push(RuntimeValue::I32(flag));
    }

    fn gt_s<T>(&mut self)
    where
        T: From<RuntimeValue> + PartialOrd,
    {
        let (a, b) = self.pop_lr::<T>();
        let flag = if a > b { 1 } else { 0 };
        self.value_stack.push(RuntimeValue::I32(flag));
    }

    fn gt_u<T>(&mut self)
    where
        T: From<RuntimeValue> + PartialOrd,
    {
        let (a, b) = self.pop_lr::<T>();
        let flag = if a > b { 1 } else { 0 };
        // TODO unsigned
        self.value_stack.push(RuntimeValue::I32(flag));
    }

    fn le_s<T>(&mut self)
    where
        T: From<RuntimeValue> + PartialOrd,
    {
        let (a, b) = self.pop_lr::<T>();
        let flag = if a <= b { 1 } else { 0 };
        self.value_stack.push(RuntimeValue::I32(flag));
    }

    fn le_u<T>(&mut self)
    where
        T: From<RuntimeValue> + PartialOrd,
    {
        let (a, b) = self.pop_lr::<T>();
        let flag = if a <= b { 1 } else { 0 };
        // TODO unsigned
        self.value_stack.push(RuntimeValue::I32(flag));
    }

    fn ge_s<T>(&mut self)
    where
        T: From<RuntimeValue> + PartialOrd,
    {
        let (a, b) = self.pop_lr::<T>();
        let flag = if a >= b { 1 } else { 0 };
        self.value_stack.push(RuntimeValue::I32(flag));
    }

    fn ge_u<T>(&mut self)
    where
        T: From<RuntimeValue> + PartialOrd,
    {
        let (a, b) = self.pop_lr::<T>();
        let flag = if a >= b { 1 } else { 0 };
        // TODO unsigned
        self.value_stack.push(RuntimeValue::I32(flag));
    }
}
