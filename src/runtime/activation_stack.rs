use crate::runtime::RuntimeError;
use crate::runtime::RuntimeValue;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ActivationStack(Vec<Activation>);

pub type Locals = HashMap<usize, RuntimeValue>;

impl ActivationStack {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn init(function_index: usize, locals_vec: Vec<RuntimeValue>) -> Self {
        let mut locals: Locals = HashMap::new();

        for i in 0..locals_vec.len() {
            locals.insert(i, locals_vec[i]);
        }

        Self(vec![Activation::new(function_index, locals)])
    }

    pub fn pc(&mut self) -> Result<usize, RuntimeError> {
        let activation = match self.last_mut() {
            None => return Err(RuntimeError::ExpectActivationStack),
            Some(v) => v,
        };

        Ok(activation.pc)
    }

    pub fn pop(&mut self) -> Option<Activation> {
        self.0.pop()
    }

    pub fn push(&mut self, activation: Activation) {
        self.0.push(activation)
    }

    pub fn set_pc(&mut self, pc: usize) -> Result<(), RuntimeError> {
        let activation = match self.last_mut() {
            None => return Err(RuntimeError::ExpectActivationStack),
            Some(v) => v,
        };
        activation.pc = pc;
        Ok(())
    }

    pub fn increment_pc(&mut self) -> Result<(), RuntimeError> {
        let activation = match self.last_mut() {
            None => return Err(RuntimeError::ExpectActivationStack),
            Some(v) => v,
        };

        activation.pc += 1;

        Ok(())
    }

    pub fn get_local(&mut self, i: usize) -> Result<&RuntimeValue, RuntimeError> {
        let locals = match self.locales_mut() {
            None => return Err(RuntimeError::ExpectActivationStack),
            Some(v) => v,
        };

        match locals.get(&i) {
            None => Err(RuntimeError::NotFound("local".to_string())),
            Some(v) => Ok(v),
        }
    }

    pub fn set_local(&mut self, index: usize, v: RuntimeValue) -> Result<(), RuntimeError> {
        let locals = match self.locales_mut() {
            None => return Err(RuntimeError::ExpectActivationStack),
            Some(v) => v,
        };

        locals.insert(index, v);

        Ok(())
    }

    pub fn last(&self) -> Option<&Activation> {
        self.0.last()
    }

    fn last_mut(&mut self) -> Option<&mut Activation> {
        self.0.last_mut()
    }

    fn locales_mut(&mut self) -> Option<&mut Locals> {
        self.last_mut().map(|a| &mut a.locals)
    }
}

#[derive(Debug)]
pub struct Activation {
    pub pc: usize,
    pub function_index: usize,

    pub locals: Locals,
}

impl Activation {
    pub fn new(function_index: usize, locals: Locals) -> Self {
        Self {
            function_index,
            locals,
            pc: 0,
        }
    }
}
