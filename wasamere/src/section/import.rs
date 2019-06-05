use crate::types::index::*;

use crate::leb_u32;
use crate::parser::{PResult, Parse};
use crate::types::{GlobalType, Limit, TableType};

use crate::StructNom;

use nom::le_u8;

#[derive(Debug, Clone, PartialEq, Parse, StructNom)]
pub struct ImportSection(pub Vec<Import>);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct Import {
    module: String,
    name: String,
    desc: ImportDesc,
}

impl Parse for Import {
    fn parse(input: &[u8]) -> PResult<Import> {
        parse_import(input)
    }
}

#[derive(Debug, Clone, PartialEq, StructNom)]
#[switch(le_u8)]
pub enum ImportDesc {
    #[byte(0x00)]
    Func(TypeIdx),
    #[byte(0x01)]
    Table(TableType),
    #[byte(0x02)]
    Mem(Limit),
    #[byte(0x03)]
    Global(GlobalType),
}

named!(
    pub parse_import<Import>,
    do_parse!(
        module: call!(String::parse)
            >> name: call!(String::parse)
            >> desc: switch!(le_u8,
            0x00 => do_parse!(
                index: call!(TypeIdx::parse) >>
                (ImportDesc::Func(index))
            ) |
            0x01 => do_parse!(
                tabletype: call!(TableType::parse) >>
                (ImportDesc::Table(tabletype))
            ) |
            0x02 => do_parse!(
                memtype: call!(Limit::parse) >>
                (ImportDesc::Mem(memtype))
            ) |
            0x03 => do_parse!(
                globaltype: call!(GlobalType::parse) >>
                (ImportDesc::Global(globaltype))
            ))
            >> (Import {
                module,
                name,
                desc
            })
    )
);
