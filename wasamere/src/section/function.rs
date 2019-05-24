use crate::leb_u32;
use crate::parser::parse_functype;
use crate::types::FuncType;
use crate::types::index::{FuncIdx, ParseIndex};

#[derive(Debug, Clone, PartialEq)]
pub struct FuncSection(pub Vec<FuncIdx>);

named!(
    pub parse_funcsec<FuncSection>,
    do_parse!(
        length: call!(leb_u32)
            >> indicies: count!(FuncIdx::parse_index, length as usize)
            >> (FuncSection(indicies))
    )
);
