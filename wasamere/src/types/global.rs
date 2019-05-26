use crate::parser::Parse;
use crate::types::{ValType, Mut};
use crate::instr::Expression;
use nom::le_u8;

#[derive(Debug, Copy, Clone, PartialEq, Parse)]
pub struct GlobalType(pub ValType, pub Mut);

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct Global(pub GlobalType, pub Expression);

// named!(pub parse_global<Global>,
//     do_parse!(
//         globaltype: call!(parse_globaltype) >>
//         init: call!(parse_expression) >>
//         (Global(globaltype, init))
//     )
// );

// named!(
//     pub parse_globaltype<GlobalType>,
//     do_parse!(
//         valtype: call!(ValType::parse)
//             >> muta: call!(Mut::parse)
//             >> (GlobalType(valtype, muta))
//     )
// );