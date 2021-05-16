#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ValueType {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncType {
    pub args: Vec<ValueType>,
    pub results: Vec<ResultType>,
}
