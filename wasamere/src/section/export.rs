use crate::types::index::*;

use crate::parser::{parse_vec, PResult, Parse};
// use crate::types::{};
use crate::leb_u32;
use crate::StructNom;

use nom::le_u8;

#[derive(Debug, Clone, PartialEq, Parse, StructNom)]
pub struct ExportSection(pub Vec<Export>);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct Export {
    pub name: String,
    pub desc: ExportDesc,
}

impl Parse for Export {
    fn parse(input: &[u8]) -> PResult<Export> {
        parse_export(input)
    }
}

#[derive(Debug, Clone, PartialEq, StructNom)]
#[switch(le_u8)]
pub enum ExportDesc {
    #[byte(0x00)]
    Func(TypeIdx),
    #[byte(0x01)]
    Table(TableIdx),
    #[byte(0x02)]
    Mem(MemIdx),
    #[byte(0x03)]
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
