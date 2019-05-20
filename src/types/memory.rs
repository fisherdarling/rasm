use crate::types::{
    expression::Expr,
    index::{FuncIdx, MemIdx, TableIdx},
};

// TODO: ConstExpr
#[derive(Debug, Clone)]
pub struct DataSegment {
    tableidx: TableIdx,
    offset: Expr,
    init: Vec<FuncIdx>,
}

// TODO: ConstExpr
#[derive(Debug, Clone)]
pub struct ElementSegment {
    data: MemIdx,
    offset: Expr,
    init: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Memory {}

#[derive(Debug, Clone)]
pub struct Table {
    kind: TableType,
    limits: Limit,
    // elems: Vec<
}

#[derive(Debug, Clone)]
pub enum TableType {
    FuncRef,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Limit {
    min: u32,
    max: Option<u32>,
}
