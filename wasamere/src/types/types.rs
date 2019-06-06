use self::index::*;
use crate::instr::Expression;

use crate::StructNom;

use nom::{le_u8, IResult};

use crate::leb_u32;

#[derive(Debug, Copy, Clone, PartialEq, StructNom)]
#[switch(le_u8)]
pub enum ValType {
    #[byte(0x7F)]
    I32,
    #[byte(0x7E)]
    I64,
    #[byte(0x7D)]
    F32,
    #[byte(0x7C)]
    F64,
}

#[derive(Debug, Copy, Clone, PartialEq, StructNom)]
#[switch(le_u8)]
pub enum ResType {
    #[byte(0x7F)]
    I32,
    #[byte(0x7E)]
    I64,
    #[byte(0x7D)]
    F32,
    #[byte(0x7C)]
    F64,
    #[byte(0x40)]
    Unit,
}

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct FuncType(#[tag(0x60)] pub Vec<ValType>, pub Vec<ResType>);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct Function(#[call(leb_u32)] pub Locals, pub Expression);

#[derive(Debug, Copy, Clone, PartialEq, StructNom)]
#[parser = "parse_limit"]
pub struct Limit {
    pub min: u32,
    pub max: Option<u32>,
}

named!(
    pub parse_limit<Limit>,
    map!(
        switch!(le_u8,
            0x00 => count!(leb_u32, 1) |
            0x01 => count!(leb_u32, 2)
        ),
        |s| if s.len() == 1 {
            Limit {
                min: s[0],
                max: None,
            }
        } else {
            Limit {
                min: s[0],
                max: Some(s[1]),
            }
        }
    )
);

#[derive(Debug, Copy, Clone, PartialEq, StructNom)]
#[switch(le_u8)]
pub enum ElemType {
    #[byte(0x70)]
    FuncRef,
}

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct Element(pub TableIdx, pub Expression, pub Vec<FuncIdx>);

#[derive(Debug, Copy, Clone, PartialEq, StructNom)]
pub struct TableType(pub ElemType, pub Limit);

#[derive(Debug, Copy, Clone, PartialEq, StructNom)]
pub struct GlobalType(pub ValType, pub Mut);

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct Global(pub GlobalType, pub Expression);

#[derive(Debug, Copy, Clone, PartialEq, StructNom)]
#[switch(le_u8)]
pub enum Mut {
    #[byte(0x00)]
    Const,
    #[byte(0x01)]
    Var,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Locals(#[debug] pub Vec<ValType>);

impl StructNom for Locals {
    fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        let mut values = Vec::new();

        let (input, ()) = do_parse!(
            input,
            num: call!(le_u8) >>
            // value!({println!("Num locals {}", num)}) >>    
            count!(do_parse!(
                inner_num: call!(le_u8) >>
                // value!({println!("inner_num {}", num)}) >>    
                val: call!(ValType::nom) >>
                ({
                    for _i in 0..inner_num {
                        values.push(val.clone());
                    }
                })
            ), num as usize) >>
            (())
        )?;

        // println!("Input after parsing locals: {:?}", input);

        Ok((input, Locals(values)))
    }
}

#[derive(Debug, Clone, PartialEq, StructNom)]
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
    impl_index!(Align);
    impl_index!(Offset);
}
