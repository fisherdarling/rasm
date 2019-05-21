use crate::types::*;

pub type ResType = Option<ValueType>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GlobalType {
    modify: Mut,
    kind: ValueType,
}

impl GlobalType {
    pub fn new(modify: Mut, kind: ValueType) -> Self {
        Self { modify, kind }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mut {
    Const,
    Var,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ValueType {
    Int32,
    Int64,
    Float32,
    Float64,
}

pub enum Type {
    Global(GlobalType),
    ResType(ResType),
    ValueType(ValueType),
    ResType(ResType),
    MemoryType(memory::MemType),
    FuncType(function::FuncType),
    Table(memory::ElemType),
}
