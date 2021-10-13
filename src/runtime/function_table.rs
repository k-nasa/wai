use alloc::vec::Vec;

use crate::module::Module;
use crate::types::*;

pub struct FunctionTable(Vec<Function>);

impl FunctionTable {
    fn empty() -> Self {
        Self(vec![])
    }

    pub fn from_module(m: &Module) -> Self {
        let funcs = match m.function_section.as_ref() {
            Some(c) => c,
            None => return Self::empty(),
        };

        let types = match m.type_section.as_ref() {
            Some(c) => c,
            None => return Self::empty(),
        };

        let codes = match m.code_section.as_ref() {
            Some(c) => c,
            None => return Self::empty(),
        };

        let mut f = vec![];

        // NOTE codes, funcの長さは同じはず。なのでどれの長さを取ってループを回しても良い
        for i in 0..codes.bodies.len() {
            let type_index = match funcs.types.get(i) {
                None => return Self::empty(),
                Some(v) => v,
            };

            let func_body = match codes.bodies.get(i) {
                None => return Self::empty(),
                Some(v) => v,
            };

            let t = match types.entries.get(*type_index as usize) {
                None => return Self::empty(),
                Some(v) => v,
            };

            f.push(Function::new(
                t.params.clone(),
                t.returns.clone(),
                func_body.code.clone(),
            ))
        }

        Self(f)
    }

    pub fn get(&self, i: usize) -> Option<&Function> {
        self.0.get(i)
    }
}

pub struct Function {
    pub params: Vec<ValueType>,
    pub returns: Vec<ValueType>,
    pub code: Vec<Instruction>,
}

impl Function {
    pub fn new(params: Vec<ValueType>, returns: Vec<ValueType>, code: Vec<Instruction>) -> Self {
        Self {
            params,
            returns,
            code,
        }
    }
}
