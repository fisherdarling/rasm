use crate::leb_u32;
use crate::parser::parse_functype;
use crate::types::FuncType;

#[derive(Debug, Clone, PartialEq)]
pub struct FuncSection(pub Vec<FuncType>);

named!(
    pub parse_funcsec<FuncSection>,
    do_parse!(
        length: call!(leb_u32)
            >> functypes: count!(parse_functype, length as usize)
            >> (FuncSection(functypes))
    )
);
