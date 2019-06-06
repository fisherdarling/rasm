mod types;

pub use types::*;

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::types::index::*;
    use super::types::*;
    use crate::instr::{Expression, Instr};
    use crate::test_parse;
    use crate::leb_u32;

    use crate::StructNom;

    static VALTYPES: &[u8] = &[0x7F, 0x7E, 0x7D, 0x7C];

    // static LARGE_FUNC: &[u8] = &[0x00, 0x61, 0x73, 0x6d, // Magic Number "\00asm"
    //                              0x01, 0x00, 0x00, 0x00, // Version: 1

    static SMALL_MODULE: &[u8] = &[
        0x00, 0x61, 0x73, 0x6d, // Magic Number: "\00asm"
        0x01, 0x00, 0x00, 0x00, // Version: 1

        // Section [1]: Type | Length: 7
        0x01, 0x07, 
        0x01, // Num functypes 
        // FuncType: 0x60 | Params: [i32, i32], Res: [i32]
        0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, 
        
        // Section [3]: Function | Length: 7
        0x03, 0x02, 
        0x01, // Num FuncIdx
        0x00, // FuncIdx(0)
        
        // Section [7]: Export | Length: 11
        0x07, 0x0b, 
        0x01, // Num Exports
        0x07, // Name Len: 7 (0x07) 
        0x67, 0x65, 0x74, 0x43, 0x65, 0x6c, 0x6c, // "getCell"
        0x00, // Export Kind: Func
        0x00, // FuncIdx(0)
        
        // Section [10]: Code | Length: 0x27
        0x0a, 0x27, 
        0x01, // Num Functions

        // Function [0]:
        0x25, // Function Size
        0x00, // Num Locals
        // Block: [i32]
        0x02, 0x7f, 
        0x41, 0x00, // i32.const: 0
        0x41, 0x32, // i32.const: 0x32
        0x20, 0x00, // local.get: 0
        0x10, 0x05, // call: FuncIdx(5)
        0x41, 0x00, // i32.const: 0
        0x41, 0x32, // i32.const: 0x32
        0x20, 0x01, // local.get: 1
        0x10, 0x0a, // call: FuncIdx(10)
        0x71,       // i32.add
        0x0b,       // end (block)
        // If: [i32]
        0x04, 0x7f, 
        0x20, 0x00,       // local.get: 0
        0x20, 0x01,       // local.get: 1
        0x10, 0x0f,       // call: FuncIdx(15)
        0x2d, 0x00, 0x00, // i32.load8_u { align: 0, offset: 0 }
        // Else
        0x05, 
        0x41, 0x00, // i32.const: 0
        0x0b,       // end (else)
        0x0b,       // end (if)
    ];

    static LARGE_FUNC: &[u8] = &[
        0x25, // Function Size
        0x00, // Num Locals
        // Block: [i32]
        0x02, 0x7f, 
        0x41, 0x00, // i32.const: 0
        0x41, 0x32, // i32.const: 0x32
        0x20, 0x00, // local.get: 0
        0x10, 0x05, // call: FuncIdx(5)
        0x41, 0x00, // i32.const: 0
        0x41, 0x32, // i32.const: 0x32
        0x20, 0x01, // local.get: 1
        0x10, 0x0a, // call: FuncIdx(10)
        0x71,       // i32.add
        0x0b,       // end (block)
        // If: [i32]
        0x04, 0x7f, 
        0x20, 0x00,       // local.get: 0
        0x20, 0x01,       // local.get: 1
        0x10, 0x0f,       // call: FuncIdx(15)
        0x2d, 0x00, 0x00, // i32.load8_u { align: 0, offset: 0 }
        // Else
        0x05, 
        0x41, 0x00, // i32.const: 0
        0x0b,       // end (else)
        0x0b,       // end (if)
        ];

    #[test]
    fn broken_large_func() {
        let input = LARGE_FUNC;

        let (input, func_size) = leb_u32(input).unwrap();
        let (input, locals) = Locals::nom(input).unwrap();
        let (input, block) = Instr::nom(input).unwrap();
        let (_input, if_stmt) = Instr::nom(input).unwrap();

        println!("{:x?}, {:#?}, {:#?}", func_size, locals, block);
        println!("{:#?}", if_stmt);

    }

    test_parse!(parse_valtypes, 
        Vec<ValType> => vec![ValType::I32, ValType::I64, ValType::F32, ValType::F64], 
        &[&[0x04], VALTYPES].concat()
    );

    test_parse!(parse_restypes,
        Vec<ResType> => vec![ResType::I32, ResType::I64, ResType::F32, 
                            ResType::F64, ResType::Unit],
        &[&[0x05], VALTYPES, &[0x40]].concat()
    );

    test_parse!(parse_functype,
        FuncType => FuncType(vec![ValType::I32, ValType::I32], vec![ResType::I32]),
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
            Instr::End,
        ])),
        &[0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b]
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

    #[test]
    fn large_func_body() {

        let input = LARGE_FUNC;
        let (_input, function) = Function::nom(input).expect("Unable to parse code");

        println!("{:#?}", function);
    }

}
