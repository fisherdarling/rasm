use crate::types::index::TypeIdx;
use crate::types::{GlobalType, Limit, TableType};

use crate::StructNom;

use nom::le_u8;

#[derive(Debug, Default, Clone, PartialEq, StructNom)]
pub struct ImportSection(pub Vec<Import>);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub desc: ImportDesc,
}

#[derive(Debug, Clone, PartialEq, StructNom)]
#[snom(switch = le_u8)]
pub enum ImportDesc {
    #[snom(val = 0x00)]
    Func(TypeIdx),
    #[snom(val = 0x01)]
    Table(TableType),
    #[snom(val = 0x02)]
    Mem(Limit),
    #[snom(val = 0x03)]
    Global(GlobalType),
}
