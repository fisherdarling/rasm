use crate::parser::{parse_globaltype, parse_expression};
use crate::types::Global;
use crate::leb_u32;

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalSection(Vec<Global>);

named!(pub parse_global<Global>,
    do_parse!(
        globaltype: call!(parse_globaltype) >>
        init: call!(parse_expression) >>
        (Global(globaltype, init))
    )
);

named!(pub parse_globalsec<GlobalSection>,
    do_parse!(
        length: call!(leb_u32) >>
        globals: count!(parse_global, length as usize) >>
        (GlobalSection(globals))
    )
);