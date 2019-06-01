#![feature(custom_attribute)]

#[macro_use]
extern crate wasamere_derive;

#[macro_use]
extern crate nom;

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
    // #[parser()]
    expr: Expression,
    // #[tag(0x10)]
    sig: Signature,
}

fn main() {
    use crate::parser::Parse;
    
    let input: &[u8] = &[0x10, 0x11, 0x12];

    let (input, res) = FieldAttr::parse(input).unwrap();

    println!("{:?}", res);
}
