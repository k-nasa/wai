use crate::types::ValueType;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RuntimeValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    V128(u128),
}

impl RuntimeValue {
    pub fn to_type(&self) -> ValueType {
        use RuntimeValue::*;

        match self {
            I32(_) => ValueType::I32,
            I64(_) => ValueType::I64,
            F32(_) => ValueType::F32,
            F64(_) => ValueType::F64,
            _ => unreachable!(),
        }
    }
}

impl From<RuntimeValue> for i32 {
    fn from(v: RuntimeValue) -> i32 {
        use RuntimeValue::*;
        match v {
            I32(x) => x as i32,
            I64(x) => x as i32,
            F32(x) => x as i32,
            F64(x) => x as i32,
            V128(x) => x as i32,
        }
    }
}

impl From<RuntimeValue> for u32 {
    fn from(v: RuntimeValue) -> u32 {
        use RuntimeValue::*;
        match v {
            I32(x) => x as u32,
            I64(x) => x as u32,
            F32(x) => x as u32,
            F64(x) => x as u32,
            V128(x) => x as u32,
        }
    }
}

impl From<RuntimeValue> for i64 {
    fn from(v: RuntimeValue) -> i64 {
        use RuntimeValue::*;
        match v {
            I32(x) => x as i64,
            I64(x) => x as i64,
            F32(x) => x as i64,
            F64(x) => x as i64,
            V128(x) => x as i64,
        }
    }
}

impl From<RuntimeValue> for usize {
    fn from(v: RuntimeValue) -> usize {
        use RuntimeValue::*;
        match v {
            I32(x) => x as usize,
            I64(x) => x as usize,
            F32(x) => x as usize,
            F64(x) => x as usize,
            V128(x) => x as usize,
        }
    }
}

impl From<RuntimeValue> for bool {
    fn from(v: RuntimeValue) -> bool {
        use RuntimeValue::*;
        match v {
            I32(x) => x as u32 != 0,
            I64(x) => x as u32 != 0,
            F32(x) => x as u32 != 0,
            F64(x) => x as u32 != 0,
            V128(x) => x as u32 != 0,
        }
    }
}
impl From<RuntimeValue> for f32 {
    fn from(v: RuntimeValue) -> f32 {
        use RuntimeValue::*;
        match v {
            I32(x) => x as f32,
            I64(x) => x as f32,
            F32(x) => x as f32,
            F64(x) => x as f32,
            V128(x) => x as f32,
        }
    }
}

impl From<RuntimeValue> for f64 {
    fn from(v: RuntimeValue) -> f64 {
        use RuntimeValue::*;
        match v {
            I32(x) => x as f64,
            I64(x) => x as f64,
            F32(x) => x as f64,
            F64(x) => x as f64,
            V128(x) => x as f64,
        }
    }
}

impl From<i32> for RuntimeValue {
    fn from(x: i32) -> Self {
        RuntimeValue::I32(x)
    }
}

impl From<u8> for RuntimeValue {
    fn from(x: u8) -> Self {
        RuntimeValue::I32(x as i32)
    }
}

impl From<i64> for RuntimeValue {
    fn from(x: i64) -> Self {
        RuntimeValue::I64(x)
    }
}

impl From<f32> for RuntimeValue {
    fn from(x: f32) -> Self {
        RuntimeValue::F32(x)
    }
}

impl From<f64> for RuntimeValue {
    fn from(x: f64) -> Self {
        RuntimeValue::F64(x)
    }
}
