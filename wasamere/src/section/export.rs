use crate::types::index::*;

use crate::parser::{parse_vec, PResult, Parse};
// use crate::types::{};
use crate::leb_u32;

use nom::le_u8;

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct ExportSection(pub Vec<Export>);

#[derive(Debug, Clone, PartialEq)]
pub struct Export {
    name: String,
    desc: ExportDesc,
}

impl Parse for Export {
    fn parse(input: &[u8]) -> PResult<Export> {
        parse_export(input)
    }
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
            name: call!(String::parse)
            >> desc: switch!(le_u8,
            0x00 => do_parse!(
                index: call!(TypeIdx::parse) >>
                (ExportDesc::Func(index))
            ) |
            0x01 => do_parse!(
                index: call!(TableIdx::parse) >>
                (ExportDesc::Table(index))
            ) |
            0x02 => do_parse!(
                index: call!(MemIdx::parse) >>
                (ExportDesc::Mem(index))
            ) |
            0x03 => do_parse!(
                index: call!(GlobalIdx::parse) >>
                (ExportDesc::Global(index))
            ))
            >> (Export {
                name,
                desc
            })
    )
);
