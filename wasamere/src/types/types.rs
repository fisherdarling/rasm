use crate::instr::Expression;
use crate::parser::{PResult, Parse};
use self::index::*;

use nom::Err as NomErr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

impl Parse for ValType {
    fn parse(input: &[u8]) -> PResult<ValType> {
        let (input, code) = u8::parse(input)?;
        
        match code {
            0x7F => Ok((input, ValType::I32)),
            0x7E => Ok((input, ValType::I64)),
            0x7D => Ok((input, ValType::F32)),
            0x7C => Ok((input, ValType::F64)),
            _ => Err(NomErr::Incomplete(nom::Needed::Unknown)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResType {
    ValType(ValType),
    Unit,
}

impl ResType {
    pub const fn i_32() -> ResType {
        ResType::ValType(ValType::I32)
    }

    pub const fn i_64() -> ResType {
        ResType::ValType(ValType::I64)
    }

    pub const fn f_32() -> ResType {
        ResType::ValType(ValType::F32)
    }

    pub const fn f_64() -> ResType {
        ResType::ValType(ValType::F64)
    }

    pub const fn unit() -> ResType {
        ResType::Unit
    }
}

impl Parse for ResType {
    fn parse(input: &[u8]) -> PResult<Self> {
        let (input, code) = u8::parse(input)?;
        
        match code {
            0x7F => Ok((input, ResType::i_32())),
            0x7E => Ok((input, ResType::i_64())),
            0x7D => Ok((input, ResType::f_32())),
            0x7C => Ok((input, ResType::f_64())),
            0x40 => Ok((input, ResType::unit())),
            _ => Err(NomErr::Incomplete(nom::Needed::Unknown)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncType(Vec<ValType>, ResType);

impl FuncType {
    pub fn new(params: Vec<ValType>, results: Vec<ResType>) -> Self {
        Self(params, *results.get(0).unwrap_or(&ResType::Unit))
    }
}

impl Parse for FuncType {
    fn parse(input: &[u8]) -> PResult<Self> {
        crate::parser::parse_functype(input)
    }
}

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct Function(pub Locals, pub Expression);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Limit {
    pub min: u32,
    pub max: Option<u32>,
}

impl Parse for Limit {
    fn parse(input: &[u8]) -> PResult<Limit> {
        crate::parser::parse_limit(input)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ElemType {
    FuncRef,
}

impl Parse for ElemType {
    fn parse(input: &[u8]) -> PResult<ElemType> {
        let (input, code) = u8::parse(input)?;
        
        match code {
            0x70 => Ok((input, ElemType::FuncRef)),
            _ => panic!("Invalid code for elemtype"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct Element(pub TableIdx, pub Expression, pub Vec<FuncIdx>);

#[derive(Debug, Copy, Clone, PartialEq, Parse)]
pub struct TableType(pub ElemType, pub Limit);

#[derive(Debug, Copy, Clone, PartialEq, Parse)]
pub struct GlobalType(pub ValType, pub Mut);

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct Global(pub GlobalType, pub Expression);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mut {
    Const,
    Var,
}

impl Parse for Mut {
    fn parse(input: &[u8]) -> PResult<Mut> {
        let (input, code) = u8::parse(input)?;
        
        match code {
            0x00 => Ok((input, Mut::Const)),
            0x01 => Ok((input, Mut::Var)),
            _ => panic!("Invalid code for mut {:x?}", code),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct Locals(pub Vec<ValType>);

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct Data(pub index::MemIdx, pub Expression, pub Vec<u8>);

pub mod index {
    use crate::impl_index;

    impl_index!(TypeIdx);
    impl_index!(FuncIdx);
    impl_index!(TableIdx);
    impl_index!(MemIdx);
    impl_index!(GlobalIdx);
    impl_index!(LocalIdx);
    impl_index!(LabelIdx);
}
