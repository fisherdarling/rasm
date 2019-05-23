use crate::parser::parse_tabletype;
use crate::types::TableType;
use crate::leb_u32;
// use crate:

#[derive(Debug, Clone, PartialEq)]
pub struct TableSection(pub Vec<TableType>);

named!(pub parse_tablesec<TableSection>,
    do_parse!(
        length: call!(leb_u32) >>
        tabletypes: count!(parse_tabletype, length as usize) >>
        (TableSection(tabletypes))
    )
);