use crate::types::Value;
use crate::error::Error;

// #[macro_export]
// macro_rules! math_binop {
//     ($kind:ident, $lhs:ident, $rhs:ident, $func:ident) => {
//         match ($lhs, $rhs) {
//             (Value::$kind(a), Value::$kind(b)) => Ok(Value::$kind(a.$func(b))),
//             _ => Err(crate::error::Error::TypeMismatch),
//         }
//     };
//     ($kind:ident, $lhs:ident, $rhs:ident, $func:ident, $cast:ty) => {
//         match ($lhs, $rhs) {
//             (Value::$kind(a), Value::$kind(b)) => Ok(Value::$kind((a as $cast).$func((b as $cast)))),
//             _ => Err(crate::error::Error::TypeMismatch),
//         }
//     };
// }


#[macro_export]
macro_rules! gen_binop {
    ($(,)? $name:ident => $func:ident : typed $($tail:tt)*) => {
        
        #[macro_export]
        macro_rules! $name {
            ($kind:ident, $lhs:ident, $rhs:ident, $cast:ty) => {
                match ($lhs, $rhs) {
                    (Value::$kind(a), Value::$kind(b)) => Ok(Value::$kind((a as $cast).$func(b as $cast).try_into().unwrap())),
                    _ => Err(crate::error::Error::TypeMismatch),
                }
            };
        }

        gen_binop! { $($tail)* }

    };
    ($(,)? $name:ident => $func:ident $($tail:tt)*) => {
        
        #[macro_export]
        macro_rules! $name {
            ($kind:ident, $lhs:ident, $rhs:ident) => {
                match ($lhs, $rhs) {
                    (Value::$kind(a), Value::$kind(b)) => Ok(Value::$kind(a.$func(b))),
                    _ => Err(crate::error::Error::TypeMismatch),
                }
            };
        }

        gen_binop! { $($tail)* }

    };
    ($(,)? $name:ident => [$op:tt] $($tail:tt)*) => {
        
        #[macro_export]
        macro_rules! $name {
            ($kind:ident, $lhs:ident, $rhs:ident) => {
                match ($lhs, $rhs) {
                    (Value::$kind(a), Value::$kind(b)) => Ok(Value::$kind(a $op b)),
                    _ => Err(crate::error::Error::TypeMismatch),
                }
            };
        }

        gen_binop! { $($tail)* }

    };
    // Relops:
    ($(,)? $name:ident ?> [$op:tt] : typed $($tail:tt)*) => {
        
        #[macro_export]
        macro_rules! $name {
            ($kind:ident, $lhs:ident, $rhs:ident, $cast:ty) => {
                match ($lhs, $rhs) {
                    (Value::$kind(a), Value::$kind(b)) => Ok(Value::from(a as $cast $op (b as $cast))),
                    _ => Err(crate::error::Error::TypeMismatch),
                }
            };
        }

        gen_binop! { $($tail)* }

    };
    ($(,)? $name:ident ?> [$op:tt] $($tail:tt)*) => {
        
        #[macro_export]
        macro_rules! $name {
            ($kind:ident, $lhs:ident, $rhs:ident) => {
                match ($lhs, $rhs) {
                    (Value::$kind(a), Value::$kind(b)) => Ok(Value::from(a $op b)),
                    _ => Err(crate::error::Error::TypeMismatch),
                }
            };
        }

        gen_binop! { $($tail)* }

    };
    ($(,)?) => {};
}

macro_rules! gen_unop {
    ($(,)? $name:ident => $func:ident $($tail:tt)*) => {
        
        #[macro_export]
        macro_rules! $name {
            ($kind:ident, $val:ident) => {
                if let Value::$kind(v) = $val {
                    Ok(Value::$kind(v.$func().into()))
                } else {
                    Err(crate::error::Error::TypeMismatch)
                }
            };
        }

        gen_unop! { $($tail)* }
    };
    ($(,)? $name:ident => [!] $($tail:tt)*) => {
        
        #[macro_export]
        macro_rules! $name {
            ($kind:ident, $val:ident) => {
                if let Value::$kind(v) = $val {
                    Ok(Value::$kind(v * -1.0))
                } else {
                    Err(crate::error::Error::TypeMismatch)
                }
            };
        }

        gen_unop! { $($tail)* }
    };
    ($(,)?) => {};
}

// TODO SHL/SHR, ROTL/ROTR, Eqz
gen_binop! {
    ieq ?> [==],
    ine ?> [!=],
    ilt_s ?> [<] : typed,
    ilt_u ?> [<],
    igt_s ?> [>] : typed,
    igt_u ?> [>],
    
    ile_s ?> [<=] : typed,
    ile_u ?> [<=],
    ige_s ?> [>=] : typed,
    ige_u ?> [>=],

    feq ?> [==],
    fne ?> [!=],
    flt ?> [<],
    fgt ?> [>],
    fle ?> [<=],
    fge ?> [>=],
    
    iadd => wrapping_add,
    isub => wrapping_sub,
    imul => wrapping_mul,
    idiv_u => wrapping_div,
    idiv_s => wrapping_div : typed,
    irem_u => wrapping_rem,
    irem_s => wrapping_rem : typed,
    iand => [&],
    ior => [|],
    ixor => [^],

    fadd => [+],
    fsub => [-],
    fmul => [*],
    fdiv => [/],
    fmin => min,
    fmax => max,
    fcopysign => copysign,
}

gen_unop! {
    iclz => leading_zeros,
    ictz => count_zeros,
    ipopcnt => count_ones,
    
    fabs => abs,
    fneg => [!],
    fceil => ceil,
    ffloor => floor,
    ftrunc => trunc,
    fnearest => round,
    fsqrt => sqrt,
}

pub fn iextend(value: Value, signed: bool) -> Result<Value, Error> {
    if let Value::I32(v) = value {
        if signed {
            Ok(Value::I64((v as i32) as u64))
        } else {
            Ok(Value::I64(v as u64))
        }
    } else {
        Err(Error::TypeMismatch)
    }
} 

#[macro_export]
macro_rules! trunc {
    ($to:ident, $from:ident, $cast:ty, $val:ident) => {
        if let Value::$from(v) = $val {
            if v.is_infinite() || v.is_nan() {
                Err(Error::UndefinedFloat)
            } else {
                Ok(Value::from(v.trunc() as $cast))
            }
        } else {
            Err(Error::TypeMismatch)
        }
    };
}

#[macro_export]
macro_rules! convert {
    ($from:ident => $to:ty, $val:ident $(, $cast:ty)?) => {
        if let Value::$from(value) = $val {
            Ok(Value::from((value $(as $cast)?) as $to))
        } else {
            Err(Error::TypeMismatch)
        }
    }
}

pub fn promote(value: Value) -> Result<Value, Error> {
    if let Value::F32(v) = value {
        Ok(Value::F64(f64::from(v)))
    } else {
        Err(Error::TypeMismatch)
    }
}

pub fn demote(value: Value) -> Result<Value, Error> {
    if let Value::F64(v) = value {
        if v.is_nan() {
            Ok(Value::F32(std::f32::NAN))
        } else if v.is_infinite() {
            if v.is_sign_positive() {
                Ok(Value::F32(std::f32::INFINITY))
            } else {
                Ok(Value::F32(std::f32::NEG_INFINITY))
            }
        } else {
            Ok(Value::F32(v as f32))
        }
    } else {
        Err(Error::TypeMismatch)
    }
}

#[macro_export]
macro_rules! reinterp {
    ($to:ident, $val:ident) => {
        if let v @ Value::$to(_) = $val.reinterpret() {
            Ok(v)
        } else {
            Err(Error::TypeMismatch)
        }
    }
}

pub fn wrap(value: Value) -> Result<Value, Error> {
    if let Value::I64(v) = value {
        Ok(Value::I32((v % u32::max_value() as u64) as u32))
    } else {
        Err(Error::TypeMismatch)
    }
}

#[macro_export]
macro_rules! shr {
    ($kind:ident, $lhs:ident, $rhs:ident $(, $cast:ty)?) => {
        match ($lhs, $rhs) {
            (Value::$kind(a), Value::I32(b)) => Ok(Value::from(a $(as $cast)? >> b)),
            _ => Err(crate::error::Error::TypeMismatch),
        } 
    }
}

#[macro_export]
macro_rules! shl {
    ($kind:ident, $lhs:ident, $rhs:ident) => {
        match ($lhs, $rhs) {
            (Value::$kind(a), Value::I32(b)) => Ok(Value::from(a << b)),
            _ => Err(crate::error::Error::TypeMismatch),
        } 
    }
}

#[macro_export]
macro_rules! rotr {
    ($kind:ident, $lhs:ident, $rhs:ident) => {
        match ($lhs, $rhs) {
            (Value::$kind(a), Value::I32(b)) => Ok(Value::from(a.rotate_right(b))),
            _ => Err(crate::error::Error::TypeMismatch),
        } 
    }
}

#[macro_export]
macro_rules! rotl {
    ($kind:ident, $lhs:ident, $rhs:ident) => {
        match ($lhs, $rhs) {
            (Value::$kind(a), Value::I32(b)) => Ok(Value::from(a.rotate_right(b))),
            _ => Err(crate::error::Error::TypeMismatch),
        } 
    }
}

// #[inline]
// pub fn shr(lhs: Value, rhs: Value, signed: bool) -> Result<Value, Error> {

// }

// #[macro_export]
// macro_rules! iadd {
//     ($kind:ident, $lhs:ident, $rhs:ident) => {
//         match ($lhs, $rhs) {
//             (Value::$kind(a), Value::$kind(b)) => Ok(Value::$kind(a.wrapping_add(b))),
//             _ => Err(crate::error::Error::TypeMismatch),
//         }
//     };
// }

// #[macro_export]
// macro_rules! idiv_s {
//     ($kind:ident, $lhs:ident, $rhs:ident, $cast:ty) => {
//         match ($lhs, $rhs) {
//             (Value::$kind(a), Value::$kind(b)) => Ok(Value::$kind((a as $cast).wrapping_div((b as $cast)))),
//             _ => Err(crate::error::Error::TypeMismatch),
//         }
//     };
// }

// #[macro_export]
// macro_rules! irem_s {
//     ($kind:ident, $lhs:ident, $rhs:ident, $cast:ty) => {
//         match ($lhs, $rhs) {
//             (Value::$kind(a), Value::$kind(b)) => Ok(Value::$kind((a as $cast).wrapping_rem((b as $cast)))),
//             _ => Err(crate::error::Error::TypeMismatch),
//         }
//     };
// }

// #[macro_export]
// macro_rules! iand {
//     ($kind:ident, $lhs:ident, $rhs:ident, $cast:ty) => {
//         math_binop!($kind, $lhs, $rhs, $cast)
//     }
// }

fn test() {
    // use crate::types::Value;

    let (a, b) = (Value::I32(5), Value::I32(5));

    // math_binop!(I32, a, b, wrapping_rem);

    iand!(I32, a, b);


    // let value = Value::I32(10);

    // let res = trunc!(I64, F32, u64, value).unwrap();
}