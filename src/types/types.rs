use crate::types::*;

pub type ResType = Option<ValueType>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GlobalType {
    modify: Mut,
    kind: ValueType,
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
    ValueType(ValueType),
    ResType(ResType),
    MemoryType(memory::MemType),
    FuncType(function::FuncType),
    Table(memory::TableType),
}