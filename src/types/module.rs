use crate::types::{
    environment::{Export, Import},
    function::{FuncType, Function},
    memory::{DataSegment, ElementSegment, Memory, Table},
    Global,
};

#[derive(Debug, Clone)]
pub struct Module {
    types: Option<Vec<FuncType>>,
    funcs: Option<Vec<Function>>,
    tables: Option<Vec<Table>>,
    mems: Option<Vec<Memory>>,
    globals: Option<Vec<Global>>,
    elem: Option<Vec<ElementSegment>>,
    data: Option<Vec<DataSegment>>,
    start: Option<Function>,
    imports: Option<Vec<Import>>,
    exports: Option<Vec<Export>>,
}
