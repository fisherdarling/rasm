use crate::types::index::*;

use crate::parser::parse_vec;
// use crate::types::{};
use crate::leb_u32;

use nom::le_u8;

#[derive(Debug, Clone, PartialEq)]
pub struct ExportSection(Vec<Export>);

#[derive(Debug, Clone, PartialEq)]
pub struct Export {
    name: String,
    desc: ExportDesc,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExportDesc {
    Func(TypeIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

named!(
    pub parse_export<Export>,
    do_parse!(
            name: call!(parse_vec::<u8>)
            >> desc: switch!(le_u8,
            0x00 => do_parse!(
                index: call!(TypeIdx::parse_index) >>
                (ExportDesc::Func(index))
            ) |
            0x01 => do_parse!(
                index: call!(TableIdx::parse_index) >>
                (ExportDesc::Table(index))
            ) |
            0x02 => do_parse!(
                index: call!(MemIdx::parse_index) >>
                (ExportDesc::Mem(index))
            ) |
            0x03 => do_parse!(
                index: call!(GlobalIdx::parse_index) >>
                (ExportDesc::Global(index))
            ))
            >> (Export {
                name: String::from_utf8_lossy(&name).to_string(),
                desc
            })
    )
);

named!(pub parse_exportsec<ExportSection>,
    do_parse!(
        length: call!(leb_u32) >>
        exports: count!(parse_export, length as usize) >>
        (ExportSection(exports))
    )
);
