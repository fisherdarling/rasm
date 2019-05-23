use nom::IResult;

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

// pub fn parse_typesec(input: &[u8]) -> IResult<&[u8], TypeSection> {
//     let (input, length) = leb_u32(input)?;

//     let (input, types) = count!(input, parse_functype, length as usize)?;

//     Ok((input, TypeSection(types)))
// }
