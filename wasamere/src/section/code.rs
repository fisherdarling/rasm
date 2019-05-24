use crate::leb_u32;
use crate::parser::{parse_vec, parse_function};
use crate::types::{Locals, ValType, Function};


#[derive(Debug, Clone, PartialEq)]
pub struct CodeSection(pub Vec<Function>);


named!(
    pub parse_locals<Locals>,
    map!(parse_vec::<ValType>, |types| Locals(types))
);


named!(
    pub parse_codesec<CodeSection>,
    do_parse!(
        length: call!(leb_u32) >>
        functions: count!(parse_function, length as usize) >>
        (CodeSection(functions))
    )
);

