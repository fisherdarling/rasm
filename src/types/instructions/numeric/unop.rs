use crate::types::instructions::numeric::Bitlen;

pub enum Unop {
    Int(Bitlen, iunop::Op),
    Float(Bitlen, funop::Op),
}

pub mod iunop {
    pub enum Op {
        Clz,
        Ctz,
        Popcnt,
    }
}

pub mod funop {
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