use crate::types::*;

pub mod binary;
pub mod error;
pub mod parser;

pub use parser::*;

pub enum AstElem {
    Module(module::Module),
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
    VecType(Vec<ValueType>),
    VecValue(Vec<Value>),
    VecByte(Vec<u8>),
}
