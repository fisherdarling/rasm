use crate::parser::parse_limit;
use crate::types::Limit;
use crate::leb_u32;

#[derive(Debug, Clone, PartialEq)]
pub struct MemSection(Vec<Limit>);


named!(pub parse_memsec<MemSection>,
    do_parse!(
        length: call!(leb_u32) >>
        memtypes: count!(parse_limit, length as usize) >>
        (MemSection(memtypes))
    )
);