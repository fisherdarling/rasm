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
                    (Value::$kind(a), Value::$kind(b)) => Ok(Value::$kind((a as $cast).$func((b as $cast)))),
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
                    (Value::$kind(a), Value::$kind(b)) => Ok(Value::from((a as $cast) $op ((b as $cast)))),
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
                    Value::$kind(v.$func())
                } else {
                    Err(crate::error::Error::TypeMismatch)
                }
            };
        }

        gen_unop! { $($tail)* }
    };
    ($(,)? $name:ident => [$op:tt] $($tail:tt)*) => {
        
        #[macro_export]
        macro_rules! $name {
            ($kind:ident, $val:ident) => {
                if let Value::$kind(v) = $val {
                    Value::$kind($op v)
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
    ffloor => flor,
    ftrunc => trunc,
    fnearest => round,
    fsqrt => sqrt,
}










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
    use crate::types::Value;

    let (a, b) = (Value::I32(5), Value::I32(5));

    // math_binop!(I32, a, b, wrapping_rem);

    iand!(I32, a, b);
}