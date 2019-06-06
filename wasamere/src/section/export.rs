use crate::types::index::*;
use crate::leb_u32;
use crate::StructNom;

use nom::le_u8;

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct ExportSection(pub Vec<Export>);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct Export {
    pub name: String,
    pub desc: ExportDesc,
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