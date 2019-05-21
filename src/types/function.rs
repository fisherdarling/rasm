use crate::types::{expression::Expr, index::TypeIdx, instructions::Instr, ValueType};

#[derive(Debug, Clone, PartialEq)]
pub struct FuncType {
    params: Option<Vec<ValueType>>,
    result: Option<Vec<ValueType>>,
}

impl FuncType {
    pub fn new(params: Option<Vec<ValueType>>, result: Option<Vec<ValueType>>) -> Self {
        FuncType { params, result }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    typeidx: TypeIdx,
    locals: Vec<ValueType>,
    body: Expr,
}
