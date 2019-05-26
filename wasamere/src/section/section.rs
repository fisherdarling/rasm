use crate::parser::Parse;
use crate::types::{Function, Data, Element, Global, Limit, TableType, FuncType};
use crate::types::index::FuncIdx;

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct CodeSection(pub Vec<Function>);

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct CustomSection(pub String, pub Vec<u8>);

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct DataSection(pub Vec<Data>);

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct ElementSection(pub Vec<Element>);

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct FuncSection(pub Vec<FuncIdx>);

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct GlobalSection(pub Vec<Global>);

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct MemSection(pub Vec<Limit>);

#[derive(Debug, Copy, Clone, PartialEq, Parse)]
pub struct StartSection(pub FuncIdx);

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct TableSection(pub Vec<TableType>);

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct TypeSection(pub Vec<FuncType>);

