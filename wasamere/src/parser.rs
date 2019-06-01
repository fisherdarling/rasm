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
    fn parse(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
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

    // println!("Parsing vec of length {}", length);

    count!(input, Parse::parse, length as usize)
}

// TODO: Figure out work around with :: for type parameters
pub fn parse_functype(input: &[u8]) -> IResult<&[u8], FuncType> {
    let (input, _) = tag!(input, &[0x60u8])?;
    let (input, params) = parse_vec::<ValType>(input)?;
    let (input, results) = parse_vec::<ResType>(input)?;

    Ok((input, FuncType::new(params, results)))
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

// #[derive(Debug, Clone, PartialEq, Parse)]
// pub enum TestParse {
//     #[byte(0x70)]
//     First(String),
//     #[byte(0x35)]
//     Second,
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn parse_testenum() {
        let input = &[0x70];

        let val = TestParse::parse(input).unwrap();

        println!("{:?}", val);
    }

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
