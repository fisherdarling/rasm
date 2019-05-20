use crate::types::*;

pub mod binary;
pub mod parser;

pub use parser::*;

pub enum AstElem {
    Global(values::Global),
    ImmValue(values::Value),
    Limit(memory::Limit),
    Function(function::Function),
    Table(memory::Table),
    Memory(memory::Memory),
    Import(env::Import),
    Export(env::Export),
    Type(types::Type),
    Index(index::Index),
    Instr(instructions::Instr),
    Name(String),
}
