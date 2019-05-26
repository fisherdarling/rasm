use crate::parser::Parse;
use crate::types::Limit;

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct MemSection(pub Vec<Limit>);