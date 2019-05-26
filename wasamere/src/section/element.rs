use crate::instr::Expression;
use crate::leb_u32;
use crate::parser::Parse;
use crate::types::index::{FuncIdx, TableIdx};

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct ElementSection(pub Vec<Element>);

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct Element(pub TableIdx, pub Expression, pub Vec<FuncIdx>);

// named!(
//     parse_element<Element>,
//     do_parse!(
//         table: call!(TableIdx::parse)
//             >> init: call!(Expression::parse)
//             >> funcs: call!(Parse::parse)
//             >> (Element(table, init, funcs))
//     )
// );

// named!(
//     pub parse_elemsec<ElementSection>,
//     do_parse!(
//         length: call!(leb_u32) >>
//         elements: count!(Elem, length as usize) >>
//         (ElementSection(elements))
//     )
// );
