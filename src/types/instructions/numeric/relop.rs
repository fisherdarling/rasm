use crate::types::instructions::numeric::{Bitlen, Signed};

pub enum Relop {
    Int(Bitlen, irelop::Op),
    Float(Bitlen, frelop::Op),
}

pub mod irelop {
    use super::Signed;
    
    pub enum Op {
        Eq,
        Ne,
        Lt(Signed),
        Gt(Signed),
        Le(Signed),
        Ge(Signed),
    }
}
    
pub mod frelop {
    pub enum Op {
        Eq,
        Ne,
        Lt,
        Gt,
        Ge
    }
}