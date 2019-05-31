#![feature(custom_attribute)]

#[macro_use]
extern crate wasamere_derive;

// #[derive(Parse)]
// pub struct TestStruct(u32, TypeIdx);

#[derive(Parse)]
pub enum Test {
    #[byte(0x55)]
    First,
    #[byte(0x55)]
    Second(String),
    #[byte(0x55)]
    Third
}

fn main() {

}