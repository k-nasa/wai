use crate::types::ValueType;

#[derive(Debug, Clone, PartialEq)]
pub struct FuncType {
    pub args: Vec<ValueType>,
    pub results: Vec<ResultType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeSection {
    pub count: u32,
    pub entries: Vec<FuncType>,
}

impl Default for TypeSection {
    fn default() -> Self {
        Self {
            count: 0,
            entries: vec![],
        }
    }
}
