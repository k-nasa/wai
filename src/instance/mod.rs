use crate::module::Module;
use crate::runtime::{error::RuntimeError, FunctionTable, Memory, Runtime, RuntimeValue};
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

        let function_table = FunctionTable::from_module(&self.module);

        let func = function_table.get(index).unwrap();

        Instance::validate(&func.params, &args)?; // argsとfunc_type.paramsの個数、型をチェックする + errorをいい感じに表示してあげたい
        let memory = Memory::new(self.init_memory()?);

        let mut runtime = Runtime::new(func.code.clone(), function_table, memory);
        let stack = runtime.execute(&args)?;

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

    fn init_memory(&self) -> Result<Vec<u8>, RuntimeError> {
        let section = self.module.data_section.as_ref();
        if section.is_none() {
            return Ok(vec![]);
        }
        let init_memory = section.unwrap().segments.get(0).unwrap().data.clone();

        Ok(init_memory)
    }

    fn validate(func_type: &[ValueType], args: &[RuntimeValue]) -> Result<(), RuntimeError> {
        let args_types: Vec<_> = args.iter().map(RuntimeValue::to_type).collect();

        let expect = func_type;
        if expect != args_types {
            return Err(RuntimeError::InvalidArgs(expect.to_vec(), args_types));
        }

        Ok(())
    }
}
