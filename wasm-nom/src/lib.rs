#![feature(custom_attribute, specialization)]
#![allow(unused)]

#[macro_use]
extern crate nom;

use structnom::{generate_structnom, StructNom};

pub mod error;
pub mod instr;
mod macros;
pub mod module;
pub mod section;
pub mod types;

use nom::{le_u8, IResult};

generate_structnom!(little);

pub trait LEB32 {}

impl<T: LEB32 + From<u32>> StructNom for T {
    fn nom(input: &[u8]) -> nom::IResult<&[u8], Self> {
        let (input, res) = leb_u32(input)?;

        Ok((input, Self::from(res)))
    }
}

impl StructNom for String {
    fn nom(input: &[u8]) -> nom::IResult<&[u8], Self> {
        let (input, bytes) = <Vec<u8>>::nom(input)?;

        let string = String::from_utf8(bytes).unwrap();

        Ok((input, string))
    }
}

pub fn leb_u32(input: &[u8]) -> IResult<&[u8], u32> {
    let (rest, byte) = le_u8(input)?;

    if byte & 0x80 == 0 {
        return Ok((rest, byte as u32));
    }

    let mut slice = rest;
    let mut result = byte as u32 & 0x7F;
    let mut shift = 7;

    loop {
        let read = le_u8(slice)?;
        slice = read.0;
        let byte = read.1;

        result |= ((byte & 0x7F) as u32) << shift;
        if shift >= 25 && (byte >> (32 - shift)) != 0 {
            panic!("Invalid LEB u32 encoding.");
        }

        shift += 7;

        if (byte & 0x80) == 0 {
            break;
        }
    }

    Ok((slice, result))
}

pub fn leb_i64(mut input: &[u8]) -> IResult<&[u8], i64> {
    let mut result: i64 = 0;
    let mut shift = 0;
    loop {
        let (new_input, byte) = le_u8(input)?;
        input = new_input;
        result |= i64::from(byte & 0x7F) << shift;
        if shift >= 57 {
            let continuation_bit = (byte & 0x80) != 0;
            let sign_and_unused_bit = ((byte << 1) as i8) >> (64 - shift);
            if continuation_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
                panic!("Invalid LEB128 encoded u64");
            }
            return Ok((input, result));
        }
        shift += 7;
        if (byte & 0x80) == 0 {
            break;
        }
    }
    let ashift = 64 - shift;
    Ok((input, (result << ashift) >> ashift))
}

pub fn leb_i32(input: &[u8]) -> IResult<&[u8], i32> {
    let (rest, byte) = le_u8(input)?;

    if byte & 0x80 == 0 {
        return Ok((rest, ((byte as i32) << 25) >> 25));
    }

    let mut slice = rest;
    let mut result = (byte & 0x7F) as i32;
    let mut shift = 7;

    loop {
        let read = le_u8(slice)?;
        slice = read.0;
        let byte = read.1;

        result |= ((byte & 0x7F) as i32) << shift;
        if shift >= 25 {
            let cont_bit = (byte & 0x80) != 0;
            let sign_and_unused_bit = (byte << 1) as i8 >> (32 - shift);

            if cont_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
                panic!("Invalid v")
            }

            return Ok((slice, result));
        }

        shift += 7;

        if (byte & 0x80) == 0 {
            break;
        }
    }

    let final_shift = 32 - shift;

    Ok((slice, (result << final_shift) >> final_shift))
}
// pub fn read_var_i32(&mut self) -> Result<i32> {
//         // Optimization for single byte i32.
//         let byte = self.read_u8()?;
//         if (byte & 0x80) == 0 {
//             return Ok(((byte as i32) << 25) >> 25);
//         }

//         let mut result = (byte & 0x7F) as i32;
//         let mut shift = 7;
//         loop {
//             let byte = self.read_u8()?;
//             result |= ((byte & 0x7F) as i32) << shift;
//             if shift >= 25 {
//                 let continuation_bit = (byte & 0x80) != 0;
//                 let sign_and_unused_bit = (byte << 1) as i8 >> (32 - shift);
//                 if continuation_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
//                     return Err(BinaryReaderError {
//                         message: "Invalid var_i32",
//                         offset: self.original_position() - 1,
//                     });
//                 }
//                 return Ok(result);
//             }
//             shift += 7;
//             if (byte & 0x80) == 0 {
//                 break;
//             }
//         }
//         let ashift = 32 - shift;
//         Ok((result << ashift) >> ashift)
// }

// pub fn read_var_u32() -> Result<u32> {
//         // Optimization for single byte i32.
//         let byte = self.read_u8()?;
//         if (byte & 0x80) == 0 {
//             return Ok(byte);
//         }

//         let mut result = byte & 0x7F;
//         let mut shift = 7;
//         loop {
//             let byte = self.read_u8()?;
//             result |= ((byte & 0x7F) as u32) << shift;
//             if shift >= 25 && (byte >> (32 - shift)) != 0 {
//                 // The continuation bit or unused bits are set.
//                 return Err(BinaryReaderError {
//                     message: "Invalid var_u32",
//                     offset: self.original_position() - 1,
//                 });
//             }
//             shift += 7;
//             if (byte & 0x80) == 0 {
//                 break;
//             }
//         }
//         Ok(result)
// }
