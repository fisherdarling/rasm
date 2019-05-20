use crate::parser::AstElem;
use crate::types::*;
use std::slice::Iter;

/// TODO: BINARY PARSER


pub static MAGIC: u32 = 0x00_61_73_6D;
pub static VERSION: u32 = 0x01_00_00_00;



pub struct Parser<'a> {
    source: &'a [u8],
    inner: Iter<'a, u8>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a [u8]) -> Parser<'a> {
        Parser {
            source,
            inner: source.iter(),
        }
    }

    pub fn next(&mut self) -> AstElem {
        AstElem::Instr(instructions::Instr::Control(
            instructions::control::Instr::Nop,
        ))
    }
}
