use crate::types::{
    index::{FuncIdx, GlobalIdx, MemIdx, TableIdx, TypeIdx},
    memory::{TableType, MemType},
    types::GlobalType,
};

#[derive(Debug, Clone)]
pub struct Import {
    module: String,
    name: String,
    desc: ImportDesc,
}

#[derive(Debug, Clone)]
pub enum ImportDesc {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

#[derive(Debug, Clone)]
pub struct Export {
    name: String,
    desc: ExportDesc,
}

#[derive(Debug, Clone)]
pub enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}
