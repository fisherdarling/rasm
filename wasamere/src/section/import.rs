use crate::types::index::TypeIdx;
use crate::leb_u32;
use crate::types::{GlobalType, Limit, TableType};

use crate::StructNom;

use nom::le_u8;

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct ImportSection(pub Vec<Import>);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct Import {
    module: String,
    name: String,
    desc: ImportDesc,
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