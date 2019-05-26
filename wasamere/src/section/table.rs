use crate::parser::Parse;
use crate::types::TableType;

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct TableSection(pub Vec<TableType>);
