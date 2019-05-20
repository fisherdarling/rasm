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

