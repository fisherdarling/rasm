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

// #[derive(Parse)]
// pub struct TestStruct(u32, TypeIdx);

#[derive(Parse)]
pub struct Signature;

#[derive(Parse)]
pub struct Expression;

#[derive(Parse)]
pub struct UnnamedField(Expression, Signature);

#[derive(Parse)]
pub struct NamedField {
    expr: Expression,
    sig: Signature,
}


#[derive(Parse)]
pub struct FieldAttr {
    #[tag(0x10)]
    expr: Expression,
    sig: Signature,
}

fn main() {
    // let input: &[u8] = &[];
}
