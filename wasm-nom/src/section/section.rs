use crate::types::index::FuncIdx;
use crate::types::{Data, Element, FuncType, Function, Global, Limit, TableType};
use crate::StructNom;
use nom::le_u8;

use crate::section::{export::ExportSection, import::ImportSection};

#[derive(Debug, Default, Clone, PartialEq, StructNom)]
pub struct CodeSection(pub Vec<Function>);

#[derive(Debug, Default, Clone, PartialEq, StructNom)]
pub struct CustomSection(pub String, pub Vec<u8>);

#[derive(Debug, Default, Clone, PartialEq, StructNom)]
pub struct DataSection(pub Vec<Data>);

#[derive(Debug, Default, Clone, PartialEq, StructNom)]
pub struct ElementSection(pub Vec<Element>);

#[derive(Debug, Default, Clone, PartialEq, StructNom)]
pub struct FuncSection(pub Vec<FuncIdx>);

#[derive(Debug, Default, Clone, PartialEq, StructNom)]
pub struct GlobalSection(pub Vec<Global>);

#[derive(Debug, Default, Clone, PartialEq, StructNom)]
pub struct MemSection(pub Vec<Limit>);

#[derive(Debug, Copy, Clone, PartialEq, StructNom)]
pub struct StartSection(pub FuncIdx);

#[derive(Debug, Default, Clone, PartialEq, StructNom)]
pub struct TableSection(pub Vec<TableType>);

#[derive(Debug, Default, Clone, PartialEq, StructNom)]
pub struct TypeSection(pub Vec<FuncType>);

crate::impl_leb32_wrapper!(Size);

#[derive(Debug, Clone, PartialEq, StructNom)]
#[snom(switch = le_u8)]
pub enum Section {
    #[snom(range(start = 0))]
    Custom(Size, CustomSection),
    Type(Size, TypeSection),
    Import(Size, ImportSection),
    Func(Size, FuncSection),
    Table(Size, TableSection),
    Mem(Size, MemSection),
    Global(Size, GlobalSection),
    Export(Size, ExportSection),
    Start(Size, StartSection),
    Element(Size, ElementSection),
    Code(Size, CodeSection),
    #[snom(range(end = 11))]
    Data(Size, DataSection),
}

impl Section {
    pub fn map_custom(&self) -> Option<&CustomSection> {
        if let Section::Custom(ref size, ref v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn map_type(&self) -> Option<&TypeSection> {
        if let Section::Type(ref size, ref v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn map_import(&self) -> Option<&ImportSection> {
        if let Section::Import(ref size, ref v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn map_func(&self) -> Option<&FuncSection> {
        if let Section::Func(ref size, ref v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn map_table(&self) -> Option<&TableSection> {
        if let Section::Table(ref size, ref v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn map_mem(&self) -> Option<&MemSection> {
        if let Section::Mem(ref size, ref v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn map_global(&self) -> Option<&GlobalSection> {
        if let Section::Global(ref size, ref v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn map_export(&self) -> Option<&ExportSection> {
        if let Section::Export(ref size, ref v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn map_start(&self) -> Option<&StartSection> {
        if let Section::Start(ref size, ref v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn map_element(&self) -> Option<&ElementSection> {
        if let Section::Element(ref size, ref v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn map_code(&self) -> Option<&CodeSection> {
        if let Section::Code(ref size, ref v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn map_data(&self) -> Option<&DataSection> {
        if let Section::Data(ref size, ref v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::*;

    const DATA: &[u8] = &[
        10, 83, 1, 81, 1, 3, 127, 65, 112, 33, 1, 3, 64, 32, 1, 65, 144, 8, 106, 34, 2, 32, 2, 40,
        2, 0, 34, 2, 32, 2, 108, 54, 2, 0, 32, 1, 65, 4, 106, 34, 1, 13, 0, 11, 65, 0, 33, 2, 65,
        112, 33, 1, 3, 64, 32, 1, 65, 144, 8, 106, 40, 2, 0, 32, 2, 106, 33, 2, 32, 1, 65, 4, 106,
        34, 3, 33, 1, 32, 3, 13, 0, 11, 32, 2, 11,
    ];

    #[test]
    fn test_code_section() {
        let input = DATA;
        let (input, code) = le_u8(input).unwrap();
        let (input, len) = le_u8(input).unwrap();

        let (input, func) = <Vec<Function>>::nom(input).unwrap();

        println!("Func: {:?}", func);
    }
}
