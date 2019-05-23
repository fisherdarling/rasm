use nom::le_u32;
use nom::le_u8;
use nom::take;
use nom::ErrorKind;
use nom::IResult;

use crate::error::Error;
use crate::instr::*;
use crate::types::index::*;
use crate::types::*;

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

// TODO: Figure out work around with :: for type parameters
pub fn parse_functype(input: &[u8]) -> IResult<&[u8], FuncType> {
    let (rest, _) = tag!(input, &[0x60u8])?;
    let (rest, params) = parse_vec::<ValType>(rest)?;
    let (rest, result) = parse_vec::<ValType>(rest)?;

    Ok((rest, FuncType::new(params, result)))
}

named!(
    parse_limit<Limit>,
    map!(
        switch!(le_u8,
            0x00 => count!(le_u32, 1) |
            0x01 => count!(le_u32, 2)
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
    parse_tabletype<TableType>,
    do_parse!(
        elemtype: map!(le_u8, |b| ElemType::from(b))
            >> limit: parse_limit
            >> (TableType(elemtype, limit))
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

pub fn parse_control(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
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

// pub fn parse_control(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
// }

pub fn parse_parametric(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
    switch!(input, value!(code),
        0x1A => value!(Instr::Drop) |
        0x1B => value!(Instr::Select)
    )
}

pub fn parse_variable(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
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

named!(
    parse_instr<Instr>,
    do_parse!(
        code: call!(le_u8) >>
        // instr: switch!(value!(code),
        instr:
            switch!(value!(code),
                // Control Instructions
                0x00..=0x11 => call!(parse_control, code) |
                // Parametric Instructions:
                0x1A..=0x1B => call!(parse_parametric, code) |
                0x20..=0x24 => call!(parse_variable, code) 
            )
            >> (instr)
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valtype_vec() {
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
}
