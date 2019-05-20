use crate::types::{expression::Expr, index::TypeIdx, instructions::Instr, ValueType};

#[derive(Debug, Clone)]
pub struct FuncType {
    params: Option<Vec<ValueType>>,
    result: Option<ValueType>,
}

#[derive(Debug, Clone)]
pub struct Function {
    typeidx: TypeIdx,
    locals: Vec<ValueType>,
    body: Expr,
}
