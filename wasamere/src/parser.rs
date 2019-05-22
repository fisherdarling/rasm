use nom::le_u32;
use nom::le_u8;
use nom::take;
use nom::IResult;
use nom::ErrorKind;

use crate::error::Error;

pub enum ValType {
    Int32,
    Int64,
    Float32,
    Float64,
}


pub static MAGIC_NUMBER: u32 = 0x00_61_73_6D;
pub static VERSION: u32 = 0x01_00_00_00;

pub const BYTE_TO_VALTYPE: fn(u8) -> ValType = |code| {
    match code {
        0x7F => ValType::Int32,
        0x7E => ValType::Int64,
        0x7D => ValType::Float32,
        0x7C => ValType::Float64,
        _ => panic!(),
    }
};

pub struct Parser<'a> {
    source: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        println!("{}, {:?}", source.len(), verify_header(source).map(|out| out.0.len()).map_err(|_| Error::InvalidHeader));

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

pub fn parse_vec<T>(data: &[u8], mapping: fn(u8) -> T) -> IResult<&[u8], &[u8]> {
    let (input, length) = le_u8(data)?;
    

    take!( input )(length);


    Ok((&[3], &[3]))
}
