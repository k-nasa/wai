use crate::runtime::RuntimeError;
use crate::runtime::RuntimeValue;

pub struct ActivationStack(Vec<Activation>);

impl ActivationStack {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn init(function_index: usize, locales: Vec<RuntimeValue>) -> Self {
        Self(vec![Activation {
            function_index,
            locales,
            pc: 0,
        }])
    }

    pub fn pc(&mut self) -> Result<usize, RuntimeError> {
        let activation = match self.last_mut() {
            None => return Err(RuntimeError::ExpectActivationStack),
            Some(v) => v,
        };

        Ok(activation.pc)
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

        match locals.get(i) {
            None => return Err(RuntimeError::NotFound(format!("local"))),
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

    fn locales_mut(&mut self) -> Option<&mut Vec<RuntimeValue>> {
        match self.last_mut() {
            None => return None,
            Some(activation) => Some(&mut activation.locales),
        }
    }
}

pub struct Activation {
    pub pc: usize,
    pub function_index: usize,

    pub locales: Vec<RuntimeValue>,
}
