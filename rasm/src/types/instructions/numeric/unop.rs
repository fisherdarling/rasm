use crate::types::instructions::numeric::Bitlen;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Unop {
    Int(Bitlen, iunop::Op),
    Float(Bitlen, funop::Op),
}

pub mod iunop {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Op {
        Clz,
        Ctz,
        Popcnt,
    }
}

pub mod funop {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Op {
        Abs,
        Neg,
        Sqrt,
        Ceil,
        Floot,
        Trunc,
        Nearest,
    }
}
