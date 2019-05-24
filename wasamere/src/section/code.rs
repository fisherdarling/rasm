use crate::leb_u32;
use crate::parser::{parse_function, parse_vec};
use crate::types::{Function, Locals, ValType};

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
