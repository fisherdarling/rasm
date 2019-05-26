use crate::parser::Parse;
use crate::types::FuncType;

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct TypeSection(pub Vec<FuncType>);

