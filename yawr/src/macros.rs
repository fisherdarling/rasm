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

#[macro_export]
macro_rules! same_type {
    ($lhs:ident, $rhs:ident) => {
        match ($lhs, $rhs) {
            (Value::I32(_), Value::I32(_)) => Ok(()),
            (Value::I64(_), Value::I64(_)) => Ok(()),
            (Value::F32(_), Value::F32(_)) => Ok(()),
            (Value::F64(_), Value::F64(_)) => Ok(()),
            _ => Err(crate::error::Error::TypeMismatch),
        }
    };
}

#[macro_export]
macro_rules! is_a {
    ($kind:ident, $id:ident) => {
        if let Value::$kind(_) = $id {
            Ok(())
        } else {
            Err(crate::error::Error::TypeMismatch)
        }
    };
    ($kind:ident, $e:expr) => {
        {
            if let v @ Value::$kind(_) = $e? {
                Ok(v)
            } else {
                Err(crate::error::Error::TypeMismatch)
            }
        }
    }
}

fn add_dummy() {
    use crate::types::Value;

    let a = Value::I32(5);
    let b = Value::I32(10);

    binop!(I32, |a, b| a + b)(a, b);
}