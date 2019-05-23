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

pub fn parse_vec<T: From<u8>>(data: &[u8]) -> IResult<&[u8], Vec<T>> {
    let (input, length) = le_u8(data)?;

    count!(input, map!(le_u8, Into::into), length as usize)
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
        switch!(take!(1),
            &[0x00u8] => count!(le_u32, 1) |
            &[0x00u8] => count!(le_u32, 2)
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
