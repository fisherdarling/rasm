use crate::parser::error::Error;
use crate::parser::AstElem;
use crate::types::*;
use std::iter::Enumerate;
use std::slice::Iter;

/// TODO: BINARY PARSER

pub type ParseResult<T> = Result<T, Error>;

pub static MAGIC: u32 = 0x00_61_73_6D;
pub static VERSION: u32 = 0x01_00_00_00;

pub struct Parser<'a> {
    source: &'a [u8],
    inner: Enumerate<Iter<'a, u8>>,
    pos: usize,
    lookahead: Option<u8>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a [u8]) -> Parser<'a> {
        let mut parser = Parser {
            source,
            inner: source.iter().enumerate(),
            pos: 0,
            lookahead: None,
        };

        parser.next_byte_opt();

        parser
    }

    fn next_byte_opt(&mut self) -> Option<u8> {
        match self.inner.next() {
            Some((idx, byte)) => {
                self.pos = idx;
                self.lookahead = Some(*byte);
            }
            None => {
                self.pos = self.source.len();
                self.lookahead = None;
            }
        }

        self.lookahead
    }

    fn next_byte(&mut self) -> ParseResult<u8> {
        let mut byte: Option<u8> = None;

        match self.inner.next() {
            Some((idx, b)) => {
                self.pos = idx;
                self.lookahead = Some(*b);
                byte = Some(*b);
            }
            None => {
                self.pos = self.source.len();
                self.lookahead = None;
            }
        }

        eof(byte)
    }

    fn scan_byte(&mut self, elem: AstElem) -> AstElem {
        self.next_byte_opt();

        elem
    }

    fn scan_u32(&mut self) -> ParseResult<u32> {
        let bytes = [
            self.next_byte()?,
            self.next_byte()?,
            self.next_byte()?,
            self.next_byte()?,
        ];

        Ok(u32::from_le_bytes(bytes))
    }

    fn scan_u64(&mut self) -> ParseResult<u64> {
        let bytes = [
            self.next_byte()?,
            self.next_byte()?,
            self.next_byte()?,
            self.next_byte()?,
            self.next_byte()?,
            self.next_byte()?,
            self.next_byte()?,
            self.next_byte()?,
        ];

        Ok(u64::from_le_bytes(bytes))
    }

    // TODO: Skipping result types for now

    // pub fn parse_vec_value(&mut self, kind: ValueType) -> Option<Vec<Value>> {
    //     if let Some(size) = self.lookahead {
    //         for 0..size {

    //         }
    //     }
    // }

    fn parse_vec_valuetype(&mut self) -> ParseResult<Vec<ValueType>> {
        let size = eof(self.lookahead)?;
        let mut types = Vec::with_capacity(size as usize);

        for _ in 0..size {
            let byte = eof(self.next_byte_opt())?;

            if let Some(vtype) = get_valtype(byte) {
                types.push(vtype);
            }
        }

        Ok(types)
    }

    fn parse_vec_byte(&mut self) -> ParseResult<Vec<u8>> {
        let size = eof(self.lookahead)?;
        let mut bytes = Vec::with_capacity(size as usize);

        for _ in 0..size {
            if let Some(next) = self.next_byte_opt() {
                bytes.push(next);
            } else {
                return Err(Error::EOF);
            }
        }

        Ok(bytes)
    }

    pub fn parse_functype(&mut self) -> ParseResult<function::FuncType> {
        let params = self.parse_vec_valuetype()?;
        let result = self.parse_vec_valuetype()?;

        Ok(function::FuncType::new(Some(params), Some(result)))
    }

    // pub fn parse_limit(&mut self) -> memory::Limit {
    //     if let Some(0x00) = self.lookahead {

    //     }
    // }

    // pub fn next(&mut self) -> AstElem {

    // }
}

#[inline]
fn eof(byte: Option<u8>) -> ParseResult<u8> {
    byte.ok_or_else(|| Error::EOF)
}

pub fn get_valtype(code: u8) -> Option<ValueType> {
    match code {
        0x7F => Some(ValueType::Int32),
        0x7E => Some(ValueType::Int64),
        0x7D => Some(ValueType::Float32),
        0x7C => Some(ValueType::Float64),
        _ => None,
    }
}
