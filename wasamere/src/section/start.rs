use crate::types::index::FuncIdx;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StartSection(FuncIdx);

named!(pub parse_startsec<StartSection>,
    map!(FuncIdx::parse_index, |idx| StartSection(idx))
);
