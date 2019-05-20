use crate::types::instructions::numeric::{Bitlen, Signed};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cvtop(Bitlen, Op, Bitlen, Option<Signed>);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Wrap,
    Extend,
    Trunc,
    Convert,
    Demote,
    Promote,
    Reinterp,
}