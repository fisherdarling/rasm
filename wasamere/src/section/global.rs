use crate::leb_u32;
use crate::parser::Parse;
use crate::types::Global;

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct GlobalSection(pub Vec<Global>);


// named!(pub parse_globalsec<GlobalSection>,
//     do_parse!(
//         length: call!(leb_u32) >>
//         globals: count!(Parse::parse, length as usize) >>
//         (GlobalSection(globals))
//     )
// );
