use crate::parser::{parse_expression, parse_vec_index};
use crate::types::index::{TableIdx, FuncIdx, ParseIndex};
use crate::instr::Expression;

pub struct ElementSection(pub Vec<Element>);

#[derive(Debug, Clone, PartialEq)]
pub struct Element(pub TableIdx, pub Expression, pub Vec<FuncIdx>);

named!(parse_element<Element>,
    do_parse!(
        table: call!(TableIdx::parse_index) >>
        init: call!(parse_expression) >>
        funcs: call!(parse_vec_index::<FuncIdx>) >>
        (Element(table, init, funcs))
    )
);

