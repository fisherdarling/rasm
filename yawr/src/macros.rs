#[macro_export]
macro_rules! binop {
    ($kind:ident, $lhs:ident, $rhs:ident, $ret:expr) => {
        match ($lhs, $rhs) {
            (Value::$kind(a), Value::I32(b)) => Ok(Value::I32($ret(a, b))),
            (Value::I64(a), Value::I64(b)) => Ok(Value::I64($ret(a, b))),
            (Value::F32(a), Value::F32(b)) => Ok(Value::F32($ret(a, b))),
            (Value::F64(a), Value::F64(b)) => Ok(Value::F64($ret(a, b))),
            _ => Err(crate::error::Error::TypeMismatch),
        }
    };
}

#[macro_export]
macro_rules! relop {
    ($lhs:ident, $rhs:ident, $ret:expr) => {
        match ($lhs, $rhs) {
            (Value::I32(a), Value::I32(b)) => Ok(Value::from($ret(a, b))),
            (Value::I64(a), Value::I64(b)) => Ok(Value::from($ret(a, b))),
            (Value::F32(a), Value::F32(b)) => Ok(Value::from($ret(a, b))),
            (Value::F64(a), Value::F64(b)) => Ok(Value::from($ret(a, b))),
            _ => Err(crate::error::Error::TypeMismatch),
        }
    };
}

fn add_dummy() {
    use crate::types::Value;

    let a = Value::I32(5);
    let b = Value::I32(10);

    binop!(I32, a, b, |a, b| a + b);
}