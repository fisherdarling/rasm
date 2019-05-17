use crate::types::{
    ValueType,
    instructions::Instr,
};

#[derive(Debug, Clone)]
pub struct FuncType {
    params: Option<Vec<ValueType>>,
    result: Option<ValueType>,
}

#[derive(Debug, Clone)]
pub struct Function {
    typeidx: u32,
    locals: Vec<ValueType>,
    body: Vec<Instr>
}
