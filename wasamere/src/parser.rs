use nom::le_u32;
use nom::le_u8;
use nom::take;
use nom::ErrorKind;
use nom::IResult;

use crate::error::Error;
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

pub fn parse_vec_byte<T: From<u8>>(data: &[u8]) -> IResult<&[u8], Vec<T>> {
    let (input, length) = le_u8(data)?;

    count!(input, map!(take!(1), |b| b[0].into()), length as usize)
}

// TODO: Figure out work around with :: for type parameters
pub fn parse_functype(input: &[u8]) -> IResult<&[u8], FuncType> {
    let (rest, _) = tag!(input, &[0x60u8])?;
    let (rest, params) = parse_vec_byte::<ValType>(rest)?;
    let (rest, result) = parse_vec_byte::<ValType>(rest)?;

    Ok((rest, FuncType::new(params, result)))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valtype_vec() {
        // length: 4
        // values: I32, I64, F32, F64
        let bytes = [0x04, 0x7F, 0x7E, 0x7D, 0x7C];

        let (rest, types) = parse_vec_byte::<ValType>(&bytes).unwrap();

        assert!(rest.is_empty());

        assert_eq!(&types, &[ValType::I32, ValType::I64, ValType::F32, ValType::F64]);
    }
}