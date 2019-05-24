use nom::le_u8;
use nom::take;
use nom::ErrorKind;
use nom::IResult;
use nom::{le_f32, le_f64, le_u64};

use crate::error::Error;
use crate::instr::*;
use crate::types::index::*;
use crate::types::*;

use crate::{leb_i32, leb_u32};

pub static MAGIC_NUMBER: u32 = 0x00_61_73_6D;
pub static VERSION: u32 = 0x01_00_00_00;

pub struct Parser<'a> {
    source: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        println!(
            "{}, {:?}",
            source.len(),
            verify_header(source)
                .map(|out| out.0.len())
                .map_err(|_| Error::InvalidHeader)
        );

        Parser { source }
    }
}

named!(
    verify_header,
    preceded!(
        tag!(MAGIC_NUMBER.to_be_bytes()),
        tag!(VERSION.to_be_bytes())
    )
);

pub fn parse_vec<T: From<u8>>(data: &[u8]) -> IResult<&[u8], Vec<T>> {
    let (input, length) = le_u8(data)?;

    count!(input, map!(le_u8, Into::into), length as usize)
}

pub fn parse_vec_index<T: ParseIndex>(data: &[u8]) -> IResult<&[u8], Vec<T>> {
    let (input, length) = le_u8(data)?;

    count!(input, T::parse_index, length as usize)
}

named!(
    pub parse_globaltype<GlobalType>,
    do_parse!(
        valtype: call!(le_u8)
            >> muta: call!(le_u8)
            >> (GlobalType(ValType::from(valtype), Mut::from(muta)))
    )
);

named!(
    pub parse_valtype<ValType>,
    map!(le_u8, |b| ValType::from(b))
);

// TODO: Figure out work around with :: for type parameters
pub fn parse_functype(input: &[u8]) -> IResult<&[u8], FuncType> {
    let (rest, _) = tag!(input, &[0x60u8])?;
    let (rest, params) = parse_vec::<ValType>(rest)?;
    let (rest, result) = parse_vec::<ValType>(rest)?;

    Ok((rest, FuncType::new(params, result)))
}

pub fn parse_function(input: &[u8]) -> IResult<&[u8], Function> {
    use crate::section::code::parse_locals;

    let (rest, num_locals) = leb_u32(input)?;
    let (rest, vec_locals) = count!(
        rest,
        map!(parse_locals, |local| local.0),
        num_locals as usize
    )?;
    let vec_valtypes: Vec<ValType> = vec_locals.into_iter().flatten().collect();

    let locals = Locals(vec_valtypes);

    let (rest, code) = parse_expression(rest)?;

    Ok((rest, Function(locals, code)))
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

named!(
    pub parse_tabletype<TableType>,
    do_parse!(
        elemtype: map!(le_u8, |b| ElemType::from(b))
            >> limit: parse_limit
            >> (TableType(elemtype, limit))
    )
);

named!(
    pub parse_expression<Expression>,
    do_parse!(
        instrs: many_till!(parse_instr, tag!(&[0x0B])) >>
        (Expression(instrs.0)) 
    )
);

named!(
    parse_block<Instr>,
    do_parse!(
        restype: call!(le_u8)
            >> instrs: many_till!(parse_instr, tag!(&[0x0B]))
            >> (Instr::Block(ResType::from(restype), instrs.0))
    )
);

named!(
    parse_loop<Instr>,
    do_parse!(
        restype: call!(le_u8)
            >> instrs: many_till!(parse_instr, tag!(&[0x0B]))
            >> (Instr::Loop(ResType::from(restype), instrs.0))
    )
);

named!(
    parse_if<Instr>,
    do_parse!(
        restype: call!(le_u8)
            >> conseq: many_till!(parse_instr, alt!(tag!(&[0x0B]) | tag!(&[0x0F])))
            >> altern:
                switch!(value!(conseq.1),
                    &[0x0F] => map!(many_till!(parse_instr, tag!(&[0x0B])), |s| s.0) |
                    &[0x0B] => value!(Vec::<Instr>::new())
                )
            >> (Instr::If(ResType::from(restype), conseq.0, altern))
    )
);

// pub fn parse_label<'a, T: index::ParseIndex>(input: &'a [u8]) -> IResult<&'a [u8], T> {
//     T::parse_index(input)
// }

pub fn parse_control_instr(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
    switch!(input, value!(code),
        // Control Instructions:
        0x00 => value!(Instr::Unreachable) |
        0x01 => value!(Instr::Nop) |
        // Block
        0x02 => call!(parse_block) |
        // Loop
        0x03 => call!(parse_loop) |
        // If
        0x04 => call!(parse_if) |
        // Br label
        0x0C => do_parse!(
            index: call!(LabelIdx::parse_index) >>
            (Instr::Br(index))
        ) |
        // Br If
        0x0D => do_parse!(
            index: call!(LabelIdx::parse_index) >>
            (Instr::BrIf(index))
        ) |
        // Br Table
        0x0E => do_parse!(
            indices: call!(parse_vec_index) >>
            index: call!(LabelIdx::parse_index) >>
            (Instr::BrTable(indices, index))
        ) |
        0x0F => value!(Instr::Return) |
        0x10 => do_parse!(
            index: call!(FuncIdx::parse_index) >>
            (Instr::Call(index))
        ) |
        0x11 => do_parse!(
            index: call!(TypeIdx::parse_index) >>
            tag!(&[0x00]) >>
            (Instr::CallIndirect(index))
        )
    )
}

// pub fn parse_control_instr(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
// }

pub fn parse_parametric_instr(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
    switch!(input, value!(code),
        0x1A => value!(Instr::Drop) |
        0x1B => value!(Instr::Select)
    )
}

pub fn parse_variable_instr(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
    switch!(input, value!(code),
        0x20 => do_parse!(
            index: call!(LocalIdx::parse_index) >>
            (Instr::LocalGet(index))
        ) |
        0x21 => do_parse!(
            index: call!(LocalIdx::parse_index) >>
            (Instr::LocalSet(index))
        ) |
        0x22 => do_parse!(
            index: call!(LocalIdx::parse_index) >>
            (Instr::LocalTee(index))
        ) |
        0x23 => do_parse!(
            index: call!(GlobalIdx::parse_index) >>
            (Instr::GlobalGet(index))
        ) |
        0x24 => do_parse!(
            index: call!(GlobalIdx::parse_index) >>
            (Instr::GlobalSet(index))
        )
    )
}

pub fn parse_mem_instr(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
    switch!(input, value!(code),
            // Memory Instructions:
        0x28 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Load((align, offset)))) |
        0x29 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Load((align, offset)))) |
        0x2A => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::F32Load((align, offset)))) |
        0x2B => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::F64Load((align, offset)))) |
        0x2C => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Load8S((align, offset)))) |
        0x2D => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Load8U((align, offset)))) |
        0x2E => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Load16S((align, offset)))) |
        0x2F => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Load16U((align, offset)))) |
        0x30 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Load8S((align, offset)))) |
        0x31 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Load8U((align, offset)))) |
        0x32 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Load16S((align, offset)))) |
        0x33 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Load16U((align, offset)))) |
        0x34 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Load32S((align, offset)))) |
        0x35 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Load32U((align, offset)))) |
        0x36 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Store((align, offset)))) |
        0x37 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Store((align, offset)))) |
        0x38 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::F32Store((align, offset)))) |
        0x39 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::F64Store((align, offset)))) |
        0x3A => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Store8((align, offset)))) |
        0x3B => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Store16((align, offset)))) |
        0x3C => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Store8((align, offset)))) |
        0x3D => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Store16((align, offset)))) |
        0x3E => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Store32((align, offset)))) |
        0x3F => do_parse!(
            tag!(&[0x00]) >>
            (Instr::MemSize)) |
        0x40 => do_parse!(
            tag!(&[0x00]) >>
            (Instr::MemGrow))
    )
}

pub fn parse_num_instr(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
    switch!(input, value!(code),
        0x41 => do_parse!(
           imm: call!(leb_u32) >>
           (Instr::I32Const(imm))  
        ) |
        0x42 => do_parse!(
           imm: call!(le_u64) >>
           (Instr::I64Const(imm))  
        ) |
        0x43 => do_parse!(
           imm: call!(le_f32) >>
           (Instr::F32Const(imm))  
        ) |
        0x44 => do_parse!(
           imm: call!(le_f64) >>
           (Instr::F64Const(imm))  
        ) |
        0x45 =>	value!(Instr::I32Eqz) |
        0x46 =>	value!(Instr::I32Eq) |
        0x47 =>	value!(Instr::I32Ne) |
        0x48 =>	value!(Instr::I32LtS) |
        0x49 =>	value!(Instr::I32LtI) |
        0x4a =>	value!(Instr::I32GtS) |
        0x4b =>	value!(Instr::I32GtU) |
        0x4c =>	value!(Instr::I32LeS) |
        0x4d =>	value!(Instr::I32LeU) |
        0x4e =>	value!(Instr::I32GeS) |
        0x4f =>	value!(Instr::I32GeU) |
        0x50 =>	value!(Instr::I64Eqz) |
        0x51 =>	value!(Instr::I64Eq) |
        0x52 =>	value!(Instr::I64Ne) |
        0x53 =>	value!(Instr::I64LtS) |
        0x54 =>	value!(Instr::I64LtI) |
        0x55 =>	value!(Instr::I64GtS) |
        0x56 =>	value!(Instr::I64GtU) |
        0x57 =>	value!(Instr::I64LeS) |
        0x58 =>	value!(Instr::I64LeU) |
        0x59 =>	value!(Instr::I64GeS) |
        0x5a =>	value!(Instr::I64GeU) |
        0x5b =>	value!(Instr::F32Eq) |
        0x5c =>	value!(Instr::F32Ne) |
        0x5d =>	value!(Instr::F32Lt) |
        0x5e =>	value!(Instr::F32Gt) |
        0x5f =>	value!(Instr::F32Le) |
        0x60 =>	value!(Instr::F32Ge) |
        0x61 =>	value!(Instr::F64Eq) |
        0x62 =>	value!(Instr::F64Ne) |
        0x63 =>	value!(Instr::F64Lt) |
        0x64 =>	value!(Instr::F64Gt) |
        0x65 =>	value!(Instr::F64Le) |
        0x66 =>	value!(Instr::F64Ge) |
        0x67 =>	value!(Instr::I32Cls) |
        0x68 =>	value!(Instr::I32Ctz) |
        0x69 =>	value!(Instr::I32Popcnt) |
        0x6a =>	value!(Instr::I32Add) |
        0x6b =>	value!(Instr::I32Sub) |
        0x6c =>	value!(Instr::I32Mul) |
        0x6d =>	value!(Instr::I32DivS) |
        0x6e =>	value!(Instr::I32DivU) |
        0x6f =>	value!(Instr::I32RemS) |
        0x70 =>	value!(Instr::I32RemU) |
        0x71 =>	value!(Instr::I32And) |
        0x72 =>	value!(Instr::I32Or) |
        0x73 =>	value!(Instr::I32Xor) |
        0x74 =>	value!(Instr::I32Shl) |
        0x75 =>	value!(Instr::I32ShrS) |
        0x76 =>	value!(Instr::I32ShrU) |
        0x77 =>	value!(Instr::I32Rotl) |
        0x78 =>	value!(Instr::I32Rotr) |
        0x79 =>	value!(Instr::I64Clz) |
        0x7a =>	value!(Instr::I64Ctz) |
        0x7b =>	value!(Instr::I64Popcnt) |
        0x7c =>	value!(Instr::I64Add) |
        0x7d =>	value!(Instr::I64Sub) |
        0x7e =>	value!(Instr::I64Mul) |
        0x7f =>	value!(Instr::I64DivS) |
        0x80 =>	value!(Instr::I64DivU) |
        0x81 =>	value!(Instr::I64RemS) |
        0x82 =>	value!(Instr::I64RemU) |
        0x83 =>	value!(Instr::I64And) |
        0x84 =>	value!(Instr::I64Or) |
        0x85 =>	value!(Instr::I64Xor) |
        0x86 =>	value!(Instr::I64Shl) |
        0x87 =>	value!(Instr::I64ShrS) |
        0x88 =>	value!(Instr::I64ShrU) |
        0x89 =>	value!(Instr::I64Rotl) |
        0x8a =>	value!(Instr::I64Rotr) |
        0x8b =>	value!(Instr::F32Abs) |
        0x8c =>	value!(Instr::F32Neg) |
        0x8d =>	value!(Instr::F32Ceil) |
        0x8e =>	value!(Instr::F32Floor) |
        0x8f =>	value!(Instr::F32Trunc) |
        0x90 =>	value!(Instr::F32Nearest) |
        0x91 =>	value!(Instr::F32Sqrt) |
        0x92 =>	value!(Instr::F32Add) |
        0x93 =>	value!(Instr::F32Sub) |
        0x94 =>	value!(Instr::F32Mul) |
        0x95 =>	value!(Instr::F32Div) |
        0x96 =>	value!(Instr::F32Min) |
        0x97 =>	value!(Instr::F32Max) |
        0x98 =>	value!(Instr::F32Copysign) |
        0x99 =>	value!(Instr::F64Abs) |
        0x9a =>	value!(Instr::F64Neg) |
        0x9b =>	value!(Instr::F64Ceil) |
        0x9c =>	value!(Instr::F64Floor) |
        0x9d =>	value!(Instr::F64Trunc) |
        0x9e =>	value!(Instr::F64Nearest) |
        0x9f =>	value!(Instr::F64Sqrt) |
        0xa0 =>	value!(Instr::F64Add) |
        0xa1 =>	value!(Instr::F64Sub) |
        0xa2 =>	value!(Instr::F64Mul) |
        0xa3 =>	value!(Instr::F64Div) |
        0xa4 =>	value!(Instr::F64Min) |
        0xa5 =>	value!(Instr::F64Max) |
        0xa6 =>	value!(Instr::F64Copysign) |
        0xa7 =>	value!(Instr::I32WrapI64) |
        0xa8 =>	value!(Instr::I32TruncF32S) |
        0xa9 =>	value!(Instr::I32TruncF32U) |
        0xaa =>	value!(Instr::I32TruncF64S) |
        0xab =>	value!(Instr::I32TruncF64U) |
        0xac =>	value!(Instr::I64ExtendI32S) |
        0xad =>	value!(Instr::I64ExtendI32U) |
        0xae =>	value!(Instr::I64TruncF32S) |
        0xaf =>	value!(Instr::I64TruncF32U) |
        0xb0 =>	value!(Instr::I64TruncF64S) |
        0xb1 =>	value!(Instr::I64TruncF64U) |
        0xb2 =>	value!(Instr::F32ConvertI32S) |
        0xb3 =>	value!(Instr::F32ConvertI32U) |
        0xb4 =>	value!(Instr::F32ConvertI64S) |
        0xb5 =>	value!(Instr::F32DemoteF64) |
        0xb6 =>	value!(Instr::F32ConvertI64U) |
        0xb7 =>	value!(Instr::F64ConvertI32S) |
        0xb8 =>	value!(Instr::F64ConvertI32U) |
        0xb9 =>	value!(Instr::F64ConvertI64S) |
        0xba =>	value!(Instr::F64ConvertI64U) |
        0xbb =>	value!(Instr::F64PromoteF32) |
        0xbc =>	value!(Instr::I32ReinterpF32) |
        0xbd =>	value!(Instr::I64ReinterpF64) |
        0xbe =>	value!(Instr::F32ReinterpI32) |
        0xbf =>	value!(Instr::F64ReinterpI64))
}

named!(
    parse_instr<Instr>,
    do_parse!(
        code: call!(le_u8) >>
        // instr: switch!(value!(code),
        instr:
            switch!(value!(code),
                // Control Instructions
                0x00..=0x11 => call!(parse_control_instr, code) |
                
                // Parametric Instructions:
                0x1A..=0x1B => call!(parse_parametric_instr, code) |
                
                // Variable Instructions:
                0x20..=0x24 => call!(parse_variable_instr, code) |
                
                // Mem Instructions:
                0x28..=0x40 => call!(parse_mem_instr, code) |

                // Numeric Instructions:
                0x41..=0xB4 => call!(parse_num_instr, code)
            )
            >> (instr)
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valtype_vec() {
        // length: 4
        // values: I32, I64, F32, F64
        let bytes = [0x04, 0x7F, 0x7E, 0x7D, 0x7C];

        let (rest, types) = parse_vec::<ValType>(&bytes).unwrap();

        assert!(rest.is_empty());

        assert_eq!(
            &types,
            &[ValType::I32, ValType::I64, ValType::F32, ValType::F64]
        );
    }

    #[test]
    fn parse_var_instr() {
        let bytes = [0x20, 0x00];

        let (rest, instr) = parse_instr(&bytes).unwrap();

        assert!(rest.is_empty());

        println!("{:?}", instr);
    }

    #[test]
    fn parse_functype_many() {
        let bytes = [0x60, 0x02, 0x7F, 0x7F, 0x01, 0x7F];

        let (rest, functype) = parse_functype(&bytes).unwrap();

        assert!(rest.is_empty());

        println!("{:?}", functype);
    }
}
