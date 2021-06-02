use crate::instruction::Instruction;
use crate::runtime::error::RuntimeError;

use crate::module::Module;
use crate::types::*;

#[derive(Debug)]
pub struct Instance {
    module: Module,
}

type ValueStack = Vec<RuntimeValue>;

impl Instance {
    pub fn new(module: Module) -> Self {
        Self { module }
    }

    pub fn invoke(
        &self,
        name: impl AsRef<str>,
        args: Vec<RuntimeValue>,
    ) -> Result<ValueStack, RuntimeError> {
        let index = self.resolve_function_name(name.as_ref());
        let index = match index {
            None => return Err(RuntimeError::NotFound(name.as_ref().to_string())),
            Some(i) => i,
        };

        let func_type = self.get_func_type(index)?;
        let func = self.get_function(index)?;

        Instance::validate(func_type, &args)?; // argsとfunc_type.paramsの個数、型をチェックする + errorをいい感じに表示してあげたい
        let stack = Instance::execute(func, &args)?;

        Ok(stack)
    }

    fn resolve_function_name(&self, name: impl AsRef<str>) -> Option<usize> {
        let export_section = &self.module.export_section;

        let exports = match export_section {
            None => return None,
            Some(e) => &e.entries,
        };

        let entry = exports.iter().find(|x| x.field_str == name.as_ref());

        entry.map(|x| x.index as usize)
    }

    fn get_function(&self, index: usize) -> Result<&FunctionBody, RuntimeError> {
        let code_section = &self.module.code_section.as_ref();
        if code_section.is_none() {
            return Err(RuntimeError::ExpectCodeSection);
        }
        let function = code_section.unwrap().bodies.get(index).unwrap();

        Ok(function)
    }

    fn get_func_type(&self, index: usize) -> Result<&FuncType, RuntimeError> {
        let type_section = &self.module.type_section.as_ref();
        if type_section.is_none() {
            return Err(RuntimeError::ExpectCodeSection); // fix error type
        }
        let types = type_section.unwrap().entries.get(index).unwrap();

        Ok(types)
    }

    fn validate(func_type: &FuncType, args: &[RuntimeValue]) -> Result<(), RuntimeError> {
        let args_types: Vec<_> = args.iter().map(RuntimeValue::to_type).collect();

        let expect = func_type.params.clone();
        if expect != args_types {
            return Err(RuntimeError::InvalidArgs(expect, args_types));
        }

        Ok(())
    }

    fn execute(func: &FunctionBody, args: &[RuntimeValue]) -> Result<ValueStack, RuntimeError> {
        let mut locals = args.clone().to_vec();

        let mut value_stack = Vec::new();
        // let mut label_stack: Vec<u8> = Vec::new();
        // let mut activation_stack: Vec<u8> = Vec::new();
        let mut skip_else_or_end = false;

        let instructions = &func.code;

        for instruction in instructions {
            // TODO flagじゃなくてlabelでいい感じにしたい
            if skip_else_or_end {
                // TODO support else

                if *instruction == Instruction::Else || *instruction == Instruction::End {
                    skip_else_or_end = false;
                }

                continue;
            }
            match instruction {
                Instruction::Nop => {}
                Instruction::GetLocal(_) => value_stack.push(locals.pop().unwrap()),
                Instruction::SetLocal(i) => {
                    let v = value_stack.pop().unwrap();
                    locals.insert(usize::from(*i), v);
                }
                Instruction::I32Add => {
                    let a = value_stack.pop().unwrap();
                    let b = value_stack.pop().unwrap();

                    value_stack.push(RuntimeValue::I32(i32::from(a) + i32::from(b)));
                }
                Instruction::If(_block_type) => {
                    if value_stack.is_empty() {
                        return Err(RuntimeError::Custom(
                            "value stack is empty, if is expected value".to_string(),
                        ));
                    }

                    let condition = bool::from(value_stack.pop().unwrap());
                    dbg!(condition);
                    if condition {
                    } else {
                        skip_else_or_end = true;
                    }
                }
                Instruction::End => {}
                Instruction::I32Const(i) => value_stack.push(RuntimeValue::I32(*i)),
                Instruction::Unexpected(op) => {
                    return Err(RuntimeError::Custom(format!(
                        "unexpected opcode: {:0x}",
                        op
                    )))
                }
                _ => {}
            }
        }
        Ok(value_stack)
    }
}
