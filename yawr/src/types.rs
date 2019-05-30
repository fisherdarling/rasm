pub use wasamere::types::*;

use crate::error::{Error, ExecResult};

use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Sub};
// pub use wasamere::

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
}

impl Add for Value {
    type Output = ExecResult<Value>;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a + b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a + b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::F32(a + b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a + b)),
            _ => Err(Error::TypeMismatch),
        }
    }
}

impl Div for Value {
    type Output = ExecResult<Value>;

    fn div(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a / b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a / b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::F32(a / b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a / b)),
            _ => Err(Error::TypeMismatch),
        }
    }
}
impl Mul for Value {
    type Output = ExecResult<Value>;

    fn mul(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a * b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a * b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::F32(a * b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a * b)),
            _ => Err(Error::TypeMismatch),
        }
    }
}
impl Sub for Value {
    type Output = ExecResult<Value>;

    fn sub(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a - b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a - b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::F32(a - b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a - b)),
            _ => Err(Error::TypeMismatch),
        }
    }
}

impl From<bool> for Value {
    fn from(other: bool) -> Self {
        Value::I32(other as u32)
    }
}

impl TryFrom<Value> for bool {
    type Error = crate::error::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::I32(c) = value {
            Ok(c != 0)
        } else {
            Err(crate::error::Error::TypeMismatch)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WasmResult {
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
    Unit,
}

impl From<Value> for WasmResult {
    fn from(value: Value) -> Self {
        match value {
            Value::I32(a) => WasmResult::I32(a),
            Value::I64(a) => WasmResult::I64(a),
            Value::F32(a) => WasmResult::F32(a),
            Value::F64(a) => WasmResult::F64(a),
        }
    }
}
