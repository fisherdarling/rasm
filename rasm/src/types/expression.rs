use crate::types::instructions::Instr;

#[derive(Debug, Clone, PartialEq)]
pub struct Expr(Vec<Instr>);
