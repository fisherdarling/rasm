use crate::types::index::*;

use crate::leb_u32;
use crate::parser::{Parse, parse_limit, parse_tabletype, parse_vec, PResult};
use crate::types::{GlobalType, Limit, TableType};

use nom::le_u8;

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct ImportSection(pub Vec<Import>);

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum ImportDesc {
    Func(TypeIdx),
    Table(TableType),
    Mem(Limit),
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
                tabletype: call!(parse_tabletype) >>
                (ImportDesc::Table(tabletype))
            ) |
            0x02 => do_parse!(
                memtype: call!(parse_limit) >>
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

named!(pub parse_importsec<ImportSection>,
    do_parse!(
        length: call!(leb_u32) >>
        imports: count!(parse_import, length as usize) >>
        (ImportSection(imports))
    )
);
