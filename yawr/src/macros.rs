#[macro_export]
macro_rules! binop {
    ($kind:ident, $ret:expr) => {
        |a, b| match (a, b) {
            (Value::$kind(a), Value::$kind(b)) => Ok(Value::$kind($ret(a, b))),
            _ => Err(crate::error::Error::TypeMismatch),
        };
    };
    ($kind:ident, $op:tt) => {
        |a, b| match (a, b) {
            (Value::$kind(a), Value::$kind(b)) => Ok(Value::$kind(a $op b)),
            _ => Err(crate::error::Error::TypeMismatch),
        };
    };
}

#[macro_export]
macro_rules! relop {
    ($kind:ident, $ret:expr) => {
        |a, b| match (a, b) {
            (Value::$kind(a), Value::$kind(b)) => Ok(Value::from($ret(a, b))),
            _ => Err(crate::error::Error::TypeMismatch),
        }
    };
    ($kind:ident, $op:tt) => {
        |a, b| match (a, b) {
            (Value::$kind(a), Value::$kind(b)) => Ok(Value::from(a $op b)),
            _ => Err(crate::error::Error::TypeMismatch),
        }
    };
    ($kind:ident, $op:tt, cast: $cast:ty) => {
        |a, b| match (a, b) {
            (Value::$kind(a), Value::$kind(b)) => Ok(Value::from((a as $cast) $op (b as $cast))),
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
macro_rules! valid_result {
    ($should_be:ident, $value:ident) => {
        match ($should_be, $value) {
            (ResType::I32, Value::I32(_)) => Ok(()),
            (ResType::I64, Value::I64(_)) => Ok(()),
            (ResType::F32, Value::F32(_)) => Ok(()),
            (ResType::F64, Value::F64(_)) => Ok(()),
            _ => Err(crate::error::Error::TypeMismatch),
        }
    }
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
    ($kind:ident, $($id:ident),+) => {
        {
            {
                let res = vec![$(is_a!($kind, $id)),+];
                let mut fail = Ok(());

                for res in res.into_iter() {
                    if res.is_err() {
                        fail = res;
                        break;
                    }
                }

                fail
            }
        }
    };
    ($kind:ident, $e:expr) => {
        if let v @ Value::$kind(_) = $e? {
            Ok(v)
        } else {
            Err(crate::error::Error::TypeMismatch)
        }
    };
}


// mod math {

// }



#[macro_export]
macro_rules! truthy {
    ($id:ident) => {
        bool::try_from($id)
    }
}

// fn add_dummy() {
//     use crate::types::Value;

//     let a = Value::I32(5);
//     let b = Value::I32(10);

//     binop!(I32, |a, b| a + b)(a, b).unwrap();
// }
