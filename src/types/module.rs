use crate::types::{
    env::{Export, Import},
    function::{FuncType, Function},
    index::FuncIdx,
    memory::{DataSegment, ElementSegment, Memory, Table},
    values::Global,
};

#[derive(Debug, Clone)]
pub struct ModuleType {
    types: Option<Vec<FuncType>>,
    funcs: Option<Vec<Function>>,
    tables: Option<Vec<Table>>,
    mems: Option<Vec<Memory>>,
    globals: Option<Vec<Global>>,
    elem: Option<Vec<ElementSegment>>,
    data: Option<Vec<DataSegment>>,
    start: Option<FuncIdx>,
    imports: Option<Vec<Import>>,
    exports: Option<Vec<Export>>,
}
