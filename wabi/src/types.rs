pub use wasm_nom::types::*;

use std::convert::TryFrom;

use std::mem::transmute;
// use std::ops::{Add, Div, Mul, Sub};
// pub use wasm_nom::

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl Value {
    pub fn default_valtype(ty: ValType) -> Value {
        match ty {
            ValType::I32 => Value::I32(Default::default()),
            ValType::I64 => Value::I64(Default::default()),
            ValType::F32 => Value::F32(Default::default()),
            ValType::F64 => Value::F64(Default::default()),
        }
    }

    pub fn reinterpret(self) -> Value {
        match self {
            Value::I32(v) => Value::F32(unsafe { transmute::<i32, f32>(v) }),
            Value::I64(v) => Value::F64(unsafe { transmute::<i64, f64>(v) }),
            Value::F32(v) => Value::I32(unsafe { transmute::<f32, i32>(v) }),
            Value::F64(v) => Value::I64(unsafe { transmute::<f64, i64>(v) }),
        }
    }
}

// impl fmt::Debug for Value {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Value::I32(v) => write!(f, "{}_i32", v),
//             Value::I64(v) => write!(f, "{}_i64", v),
//             Value::F32(v) => write!(f, "{}_f32", v),
//             Value::F64(v) => write!(f, "{}_f64", v),
//         }
//     }
// }

macro_rules! impl_value_from {
    ($to:ident, $cast:ty, $ty:ty) => {
        impl From<$ty> for Value {
            fn from(other: $ty) -> Self {
                Value::$to(other as $cast)
            }
        }
    };
}

impl_value_from!(I32, i32, u8);
impl_value_from!(I32, i32, u16);
impl_value_from!(I32, i32, u32);
impl_value_from!(I32, i32, i32);

impl_value_from!(I64, i64, u64);
impl_value_from!(I64, i64, i64);

impl_value_from!(F32, f32, f32);
impl_value_from!(F64, f64, f64);

// pub trait Arithmetic {
//     pub fn add(&self, other: &Self) -> Self;
//     pub fn sub(&self, other: &Self) -> Self;

// }

// impl Add for Value {
//     type Output = ExecResult<Value>;

//     fn add(self, rhs: Value) -> Self::Output {
//         match (self, rhs) {
//             (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a + b)),
//             (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a + b)),
//             (Value::F32(a), Value::F32(b)) => Ok(Value::F32(a + b)),
//             (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a + b)),
//             _ => Err(Error::TypeMismatch(line!())),
//         }
//     }
// }

// impl Div for Value {
//     type Output = ExecResult<Value>;

//     fn div(self, rhs: Value) -> Self::Output {
//         match (self, rhs) {
//             (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a / b)),
//             (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a / b)),
//             (Value::F32(a), Value::F32(b)) => Ok(Value::F32(a / b)),
//             (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a / b)),
//             _ => Err(Error::TypeMismatch(line!())),
//         }
//     }
// }
// impl Mul for Value {
//     type Output = ExecResult<Value>;

//     fn mul(self, rhs: Value) -> Self::Output {
//         match (self, rhs) {
//             (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a * b)),
//             (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a * b)),
//             (Value::F32(a), Value::F32(b)) => Ok(Value::F32(a * b)),
//             (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a * b)),
//             _ => Err(Error::TypeMismatch(line!())),
//         }
//     }
// }
// impl Sub for Value {
//     type Output = ExecResult<Value>;

//     fn sub(self, rhs: Value) -> Self::Output {
//         match (self, rhs) {
//             (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a - b)),
//             (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a - b)),
//             (Value::F32(a), Value::F32(b)) => Ok(Value::F32(a - b)),
//             (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a - b)),
//             _ => Err(Error::TypeMismatch(line!())),
//         }
//     }
// }

impl From<bool> for Value {
    fn from(other: bool) -> Self {
        Value::I32(other as i32)
    }
}

impl TryFrom<Value> for bool {
    type Error = crate::error::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::I32(c) = value {
            Ok(c != 0)
        } else {
            Err(crate::error::Error::TypeMismatch(line!()))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WasmResult {
    I32(i32),
    I64(i64),
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
