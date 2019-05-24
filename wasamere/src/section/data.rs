use crate::leb_u32;
use crate::parser::parse_data;
use crate::types::Data;

#[derive(Debug, Clone, PartialEq)]
pub struct DataSection(pub Vec<Data>);

named!(
    pub parse_datasec<DataSection>,
    do_parse!(
        length: call!(leb_u32) >>
        data: count!(parse_data, length as usize) >>
        (DataSection(data))
    )
);
