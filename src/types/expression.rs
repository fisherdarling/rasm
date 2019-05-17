use crate::types::instructions::Instr;

#[derive(Debug, Clone)]
pub struct Expr(Vec<Instr>);