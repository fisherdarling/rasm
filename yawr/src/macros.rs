#[macro_export]
// macro_rules! binop {
//     ($kind:ident, $lhs:ident, $rhs:ident, $ret:expr) => {
//         match ($lhs, $rhs) {
//             (Value::$kind(a), Value::$kind(b)) => Ok(Value::$kind($ret(a, b))),
//             _ => Err(crate::error::Error::TypeMismatch),
//         }
//     };
// }

#[macro_export]
macro_rules! binop {
    ($kind:ident, $ret:expr) => {
        |a, b| {
            match (a, b) {
                (Value::$kind(a), Value::$kind(b)) => Ok(Value::$kind($ret(a, b))),
                _ => Err(crate::error::Error::TypeMismatch),
            }
        };

    };
}

#[macro_export]
macro_rules! relop {
    ($kind:ident, $lhs:ident, $rhs:ident, $ret:expr) => {
        match ($lhs, $rhs) {
            (Value::$kind(a), Value::$kind(b)) => Ok(Value::from($ret(a, b))),
            _ => Err(crate::error::Error::TypeMismatch),
        }
    };
}

fn add_dummy() {
    use crate::types::Value;

    let a = Value::I32(5);
    let b = Value::I32(10);

    binop!(I32, |a, b| a + b)(a, b);
}