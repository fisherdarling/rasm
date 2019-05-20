use crate::types::instructions::numeric::{Bitlen, Signed};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Binop {
    Int(Bitlen, ibinop::Op),
    Float(Bitlen, fbinop::Op),
}

pub mod ibinop {
    use super::Signed;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Op {
        Add,
        Sub,
        Mul,
        Div(Signed),
        Rem(Signed),
        And,
        Or,
        Xor,
        Shl,
        Shr(Signed),
        Rotl,
        Rotr,
    }
}

pub mod fbinop {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Op {
        Add,
        Sub,
        Mul,
        Div,
        Min,
        Max,
        Copysign,
    }
}
