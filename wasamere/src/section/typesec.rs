use crate::leb_u32;
use crate::parser::parse_functype;
use crate::types::FuncType;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeSection(pub Vec<FuncType>);

named!(
    parse_typesec<TypeSection>,
    do_parse!(
        length: call!(leb_u32)
            >> types: count!(parse_functype, length as usize)
            >> (TypeSection(types))
    )
);
