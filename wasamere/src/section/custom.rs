use nom::le_u8;

use crate::leb_u32;


#[derive(Debug, Clone, PartialEq)]
pub struct CustomSection(pub Vec<u8>);

named!(pub parse_customsec<CustomSection>,
    do_parse!(
        length: call!(leb_u32) >>
        bytes: count!(le_u8, length as usize) >>
        (CustomSection(bytes))
    )
);