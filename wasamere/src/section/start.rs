use crate::types::index::FuncIdx;
use crate::parser::Parse;

#[derive(Debug, Copy, Clone, PartialEq, Parse)]
pub struct StartSection(pub FuncIdx);
