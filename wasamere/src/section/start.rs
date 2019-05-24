use crate::types::index::FuncIdx;
use crate::types::index::ParseIndex;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StartSection(pub FuncIdx);

named!(pub parse_startsec<StartSection>,
    map!(FuncIdx::parse_index, |idx| StartSection(idx))
);
