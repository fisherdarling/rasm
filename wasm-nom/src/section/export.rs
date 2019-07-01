use crate::types::index::*;
use crate::StructNom;

use nom::le_u8;

#[derive(Debug, Default, Clone, PartialEq, StructNom)]
pub struct ExportSection(pub Vec<Export>);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct Export {
    pub name: String,
    pub desc: ExportDesc,
}

#[derive(Debug, Clone, PartialEq, StructNom)]
#[snom(switch = le_u8)]
pub enum ExportDesc {
    #[snom(val = 0x00)]
    Func(TypeIdx),
    #[snom(val = 0x01)]
    Table(TableIdx),
    #[snom(val = 0x02)]
    Mem(MemIdx),
    #[snom(val = 0x03)]
    Global(GlobalIdx),
}
