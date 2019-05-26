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

pub type PResult<'a, T> = IResult<&'a [u8], T>;

pub trait Parse {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> where Self: Sized;
}

impl Parse for u8 {
    fn parse(input: &[u8]) -> PResult<Self> {
        let (input, val) = le_u8(input)?;

        Ok((input, val))
    }
}

impl Parse for u32 {
    fn parse(input: &[u8]) -> PResult<Self> {
        let (input, val) = leb_u32(input)?;

        Ok((input, val))
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(input: &[u8]) -> PResult<Self> {
        parse_vec(input)
    }
}

impl Parse for String {
    fn parse(input: &[u8]) -> PResult<Self> {
        let (input, bytes) = <Vec<u8>>::parse(input)?;

        Ok((input, String::from_utf8(bytes).unwrap()))
    }
}

pub fn parse_vec<T: Parse>(data: &[u8]) -> IResult<&[u8], Vec<T>> {
    let (input, length) = le_u8(data)?;

    count!(input, Parse::parse, length as usize)
}

// TODO: Figure out work around with :: for type parameters
pub fn parse_functype(input: &[u8]) -> IResult<&[u8], FuncType> {
    let (input, _) = tag!(input, &[0x60u8])?;
    let (input, params) = parse_vec::<ValType>(input)?;
    let (input, results) = parse_vec::<ResType>(input)?;

    Ok((input, FuncType::new(params, results)))
}

pub fn parse_function(input: &[u8]) -> IResult<&[u8], Function> {
    use crate::section::code::parse_locals;

    let (input, func_size) = leb_u32(input)?;
    let (input, num_locals) = leb_u32(input)?;
    let (input, vec_locals) = count!(
        input,
        map!(parse_locals, |local| local.0),
        num_locals as usize
    )?;

    println!("Vec Locals: {:?}", vec_locals);

    let vec_valtypes: Vec<ValType> = vec_locals.into_iter().flatten().collect();

    println!("Vec ValTypes: {:?}", vec_valtypes);

    let locals = Locals(vec_valtypes);

    let (input, code) = Expression::parse(input)?;

    Ok((input, Function(locals, code)))
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
    pub parse_data<Data>,
    do_parse!(
        index: call!(MemIdx::parse) >>
        offset: call!(Expression::parse) >>
        length: call!(leb_u32) >>
        init: count!(le_u8, length as usize) >>
        (Data(index, offset, init))
    )
);

named!(
    pub parse_tabletype<TableType>,
    do_parse!(
        elemtype: call!(ElemType::parse)
            >> limit: parse_limit
            >> (TableType(elemtype, limit))
    )
);

// named!(
//     pub parse_expression<Expression>,
//     do_parse!(
//         instrs: many_till!(Instr::parse, tag!(&[0x0B])) >>
//         (Expression(instrs.0)) 
//     )
// );

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

        let (rest, instr) = Instr::parse(&bytes).unwrap();

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
