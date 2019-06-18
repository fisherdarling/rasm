#[macro_export]
macro_rules! same_type {
    ($lhs:ident, $rhs:ident) => {
        match ($lhs, $rhs) {
            (Value::I32(_), Value::I32(_)) => Ok(()),
            (Value::I64(_), Value::I64(_)) => Ok(()),
            (Value::F32(_), Value::F32(_)) => Ok(()),
            (Value::F64(_), Value::F64(_)) => Ok(()),
            _ => Err(crate::error::Error::TypeMismatch(line!())),
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
            _ => Err(crate::error::Error::TypeMismatch(line!())),
        }
    };
    ($should_be:ident from $value:ident) => {
        match ($should_be, $value) {
            (ResType::I32, Value::I32(v)) => Ok(WasmResult::I32(v)),
            (ResType::I64, Value::I64(v)) => Ok(WasmResult::I64(v)),
            (ResType::F32, Value::F32(v)) => Ok(WasmResult::F32(v)),
            (ResType::F64, Value::F64(v)) => Ok(WasmResult::F64(v)),
            _ => Err(crate::error::Error::TypeMismatch(line!())),
        }
    };
}

#[macro_export]
macro_rules! is_a {
    ($kind:ident, $id:ident) => {
        if let Value::$kind(_) = $id {
            Ok(())
        } else {
            Err(crate::error::Error::TypeMismatch(line!()))
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
            Err(crate::error::Error::TypeMismatch(line!()))
        }
    };
}

#[macro_export]
macro_rules! get {
    ($from:ident, $id:ident) => {
        match $id {
            Value::$from(v) => Ok(v),
            _ => Err(crate::error::Error::TypeMismatch(line!())),
        }
    };
    ($from:ident, $e:expr) => {
        match $e {
            Value::$from(v) => Ok(v),
            _ => Err(crate::error::Error::TypeMismatch(line!())),
        }
    };
}


#[macro_export]
macro_rules! truthy {
    ($id:ident) => {
        bool::try_from($id)
    };
}

#[macro_export]
macro_rules! args {
    ($($lit:literal),*) => {
        vec![
            $(Value::from($lit)),*
        ]
    }
}