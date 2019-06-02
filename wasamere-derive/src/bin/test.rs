#![feature(custom_attribute)]

#[macro_use]
extern crate wasamere_derive;

#[macro_use]
extern crate nom;

use nom::le_u32;

mod parser {
    use nom::IResult;

    pub trait Parse {
        fn parse(input: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized;
    }
}

use nom::IResult;

#[derive(Debug, Parse)]
pub struct Signature;

#[derive(Debug, Parse)]
pub struct Expression;

#[derive(Parse)]
pub struct UnnamedField(Expression, Signature);

#[derive(Parse)]
pub struct NamedField {
    expr: Expression,
    sig: Signature,
}

#[derive(Debug, Parse)]
pub struct FieldAttr {
    expr: Expression,
    // #[tag(0x10)]
    sig: Signature,
}

use nom::le_u8;

#[derive(Debug, Parse)]
#[switch(le_u8)]
pub enum MyEnum {
    #[byte(0x11)]
    First(Expression),
    #[byte(0x01)]
    Second,
    #[byte(0x05)]
    Named {
        expr: Expression,
        sig: Signature,
    }
}

fn main() {
    use crate::parser::Parse;

    let input: &[u8] = &[0x10];

    let (input, res) = MyEnum::parse(input).unwrap();

    println!("{:?}", res);

//     do_parse!(
//         input,
//         val: switch!(le_u8, 
//             17u8 => nom::do_parse ! (
// field_0: call ! ( < Expression > :: parse ) >> ( MyEnum :: First ( field_0 )
// ) ) ) >> (val)
//     );
}
