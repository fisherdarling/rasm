use crate::error::Error;
use crate::types::Value;

// pub fn convert_offset(offset: Offset) -> usize {
//     offset.index() as usize
// }

// pub fn convert_align(offset: Offset) -> usize {
//     offset.index() as usize
// }

#[macro_export]
macro_rules! gen_binop {
    ($(,)? $name:ident => $func:ident : typed $($tail:tt)*) => {

        #[macro_export]
        macro_rules! $name {
            ($kind:ident, $lhs:ident, $rhs:ident, $cast:ty) => {
                match ($lhs, $rhs) {
                    (Value::$kind(a), Value::$kind(b)) => Ok(Value::$kind((a as $cast).$func(b as $cast).try_into().unwrap())),
                    _ => Err(crate::error::Error::TypeMismatch(line!())),
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
                    _ => Err(crate::error::Error::TypeMismatch(line!())),
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
                    _ => Err(crate::error::Error::TypeMismatch(line!())),
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
                    _ => Err(crate::error::Error::TypeMismatch(line!())),
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
                    _ => Err(crate::error::Error::TypeMismatch(line!())),
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
                    Ok(Value::$kind(v.$func().try_into().unwrap()))
                } else {
                    Err(crate::error::Error::TypeMismatch(line!()))
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
                    Err(crate::error::Error::TypeMismatch(line!()))
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
    ilt_u ?> [<] : typed,
    ilt_s ?> [<],
    igt_u ?> [>] : typed,
    igt_s ?> [>],

    ile_u ?> [<=] : typed,
    ile_s ?> [<=],
    ige_u ?> [>=] : typed,
    ige_s ?> [>=],

    feq ?> [==],
    fne ?> [!=],
    flt ?> [<],
    fgt ?> [>],
    fle ?> [<=],
    fge ?> [>=],

    iadd => wrapping_add,
    isub => wrapping_sub,
    imul => wrapping_mul,
    idiv_s => wrapping_div,
    idiv_u => wrapping_div : typed,
    irem_s => wrapping_rem,
    irem_u => wrapping_rem : typed,
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
    ictz => trailing_zeros,
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
            Ok(Value::I64(v as i64))
        } else {
            Ok(Value::I64((v as u32) as i64))
        }
    } else {
        Err(Error::TypeMismatch(line!()))
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
            Err(Error::TypeMismatch(line!()))
        }
    };
}

#[macro_export]
macro_rules! convert {
    ($from:ident => $to:ty, $val:ident $(, $cast:ty)?) => {
        if let Value::$from(value) = $val {
            Ok(Value::from((value $(as $cast)?) as $to))
        } else {
            Err(Error::TypeMismatch(line!()))
        }
    }
}

pub fn promote(value: Value) -> Result<Value, Error> {
    if let Value::F32(v) = value {
        Ok(Value::F64(f64::from(v)))
    } else {
        Err(Error::TypeMismatch(line!()))
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
        Err(Error::TypeMismatch(line!()))
    }
}

#[macro_export]
macro_rules! reinterp {
    ($to:ident, $val:ident) => {
        if let v @ Value::$to(_) = $val.reinterpret() {
            Ok(v)
        } else {
            Err(Error::TypeMismatch(line!()))
        }
    };
}

pub fn wrap(value: Value) -> Result<Value, Error> {
    if let Value::I64(v) = value {
        Ok(Value::I32((v % u32::max_value() as i64) as i32))
    } else {
        Err(Error::TypeMismatch(line!()))
    }
}

#[macro_export]
macro_rules! shr {
    ($kind:ident, $lhs:ident, $rhs:ident $(, $cast:ty)?) => {
        match ($lhs, $rhs) {
            (Value::$kind(a), Value::$kind(b)) => Ok(Value::from(a $(as $cast)? >> (b as u32))),
            _ => Err(crate::error::Error::TypeMismatch(line!())),
        }
    }
}

#[macro_export]
macro_rules! shl {
    ($kind:ident, $lhs:ident, $rhs:ident) => {
        match ($lhs, $rhs) {
            (Value::$kind(a), Value::I32(b)) => Ok(Value::from(a << (b as u32))),
            _ => Err(crate::error::Error::TypeMismatch(line!())),
        }
    };
}

#[macro_export]
macro_rules! rotr {
    ($kind:ident, $lhs:ident, $rhs:ident) => {
        match ($lhs, $rhs) {
            (Value::$kind(a), Value::I32(b)) => Ok(Value::from(a.rotate_right(b as u32))),
            _ => Err(crate::error::Error::TypeMismatch(line!())),
        }
    };
}

#[macro_export]
macro_rules! rotl {
    ($kind:ident, $lhs:ident, $rhs:ident) => {
        match ($lhs, $rhs) {
            (Value::$kind(a), Value::I32(b)) => Ok(Value::from(a.rotate_right(b as u32))),
            _ => Err(crate::error::Error::TypeMismatch(line!())),
        }
    };
}
