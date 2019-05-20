use crate::types::instructions::numeric::{Bitlen, Signed};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Relop {
    Int(Bitlen, irelop::Op),
    Float(Bitlen, frelop::Op),
}

pub mod irelop {
    use super::Signed;
    
    #[derive(Debug, Copy, Clone, PartialEq)]
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

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Op {
        Eq,
        Ne,
        Lt,
        Gt,
        Ge
    }
}