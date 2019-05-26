use crate::leb_u32;
use crate::parser::Parse;
use crate::types::index::FuncIdx;

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct FuncSection(pub Vec<FuncIdx>);
