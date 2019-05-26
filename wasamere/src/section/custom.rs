use nom::le_u8;

use crate::parser::Parse;
use crate::leb_u32;


#[derive(Debug, Clone, PartialEq, Parse)]
pub struct CustomSection(pub String, pub Vec<u8>);

// named!(pub parse_customsec<CustomSection>,
//     do_parse!(
//         name: call!(String::parse)
//         bytes: call!(<Vec<u8>>::parse) >>
//         (CustomSection(bytes))
//     )
// );