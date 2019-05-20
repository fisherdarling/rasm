use crate::types::instructions::numeric::{Bitlen, Signed};


pub struct Cvtop(Bitlen, Op, Bitlen, Option<Signed>);

pub enum Op {
    Wrap,
    Extend,
    Trunc,
    Convert,
    Demote,
    Promote,
    Reinterp,
}