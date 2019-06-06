use crate::types::index::FuncIdx;
use crate::types::{Data, Element, FuncType, Function, Global, Limit, TableType};
use crate::StructNom;
use nom::le_u8;

use crate::section::{export::ExportSection, import::ImportSection};

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct CodeSection(pub Vec<Function>);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct CustomSection(pub String, pub Vec<u8>);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct DataSection(pub Vec<Data>);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct ElementSection(pub Vec<Element>);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct FuncSection(pub Vec<FuncIdx>);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct GlobalSection(pub Vec<Global>);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct MemSection(pub Vec<Limit>);

#[derive(Debug, Copy, Clone, PartialEq, StructNom)]
pub struct StartSection(pub FuncIdx);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct TableSection(pub Vec<TableType>);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct TypeSection(pub Vec<FuncType>);

crate::impl_index!(Size);

#[derive(Debug, Clone, PartialEq, StructNom)]
#[switch(le_u8)]
pub enum Section {
    #[range_start(0)]
    Custom(Size, CustomSection),
    Type(Size, TypeSection),
    Import(Size, ImportSection),
    Func(Size, FuncSection),
    Table(Size, TableSection),
    Mem(Size, MemSection),
    Global(Size, GlobalSection),
    Export(Size, ExportSection),
    Start(Size, StartSection),
    Element(Size, ElementSection),
    Code(Size, CodeSection),
    #[range_end(11)]
    Data(Size, DataSection),
}
