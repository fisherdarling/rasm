mod types;

pub use types::*;

#[cfg(test)]
mod tests {
    use super::types::index::*;
    use super::types::*;
    use crate::parser::Parse;
    use crate::test_parse;
    use crate::instr::{Instr, Expression};

    static VALTYPES: &[u8] = &[0x7F, 0x7E, 0x7D, 0x7C];

    test_parse!(parse_valtypes, 
        Vec<ValType> => vec![ValType::I32, ValType::I64, ValType::F32, ValType::F64], 
        &[&[0x04], VALTYPES].concat()
    );

    test_parse!(parse_restypes,
        Vec<ResType> => vec![ResType::i_32(), ResType::i_64(), ResType::f_32(), 
                            ResType::f_64(), ResType::unit()],
        &[&[0x05], VALTYPES, &[0x40]].concat()
    );

    test_parse!(parse_functype,
        FuncType => FuncType(vec![ValType::I32, ValType::I32], ResType::i_32()),
        &[0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f]
    );

    test_parse!(parse_locals,
        Locals => Locals(vec![ValType::I32, ValType::I32, ValType::F64, ValType::F32]),
        &[03, 0x02, 0x7f, 0x01, 0x7c, 0x01, 0x7d]
    );

    test_parse!(parse_function,
        Function => Function(Locals(Vec::new()), Expression(vec![
            Instr::LocalGet(LocalIdx(0)),
            Instr::LocalGet(LocalIdx(1)),
            Instr::I32Add,
        ])),
        &[0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b]
    );

    test_parse!(parse_limit_min,
        Limit => Limit { min: 10, max: None },
        &[0x00, 0x0A]
    );

    test_parse!(parse_limit_both,
        Limit => Limit { min: 10, max: Some(15) },
        &[0x01, 0x0A, 0x0F]
    );

    test_parse!(parse_elemtype,
        ElemType => ElemType::FuncRef,
        &[0x70]
    );
}
