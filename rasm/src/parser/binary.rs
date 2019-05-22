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

    pub fn next_byte_opt(&mut self) -> Option<u8> {
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

    pub fn next_byte(&mut self) -> ParseResult<u8> {
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

        eof(self.lookahead)
    }

    pub fn scan_byte(&mut self, elem: AstElem) -> AstElem {
        self.next_byte_opt();

        elem
    }

    pub fn scan_u32(&mut self) -> ParseResult<u32> {
        let bytes = [
            self.next_byte()?,
            self.next_byte()?,
            self.next_byte()?,
            self.next_byte()?,
        ];

        Ok(u32::from_le_bytes(bytes))
    }

    pub fn scan_u64(&mut self) -> ParseResult<u64> {
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

    pub fn parse_vec_valuetype(&mut self) -> ParseResult<Vec<ValueType>> {
        let size = eof(self.lookahead)?;
        let mut types = Vec::with_capacity(size as usize);

        for _ in 0..size {
            types.push(self.parse_valuetype()?);
        }

        Ok(types)
    }

    pub fn parse_valuetype(&mut self) -> ParseResult<ValueType> {
        get_valtype(eof(self.lookahead)?)
    }

    pub fn parse_vec_byte(&mut self) -> ParseResult<Vec<u8>> {
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

    pub fn parse_restype(&mut self) -> ParseResult<types::ResType> {
        if let Some(0x40) = self.lookahead {
            Ok(None)
        } else {
            Ok(Some(get_valtype(eof(self.lookahead)?)?))
        }
    }

    pub fn parse_functype(&mut self) -> ParseResult<function::FuncType> {
        let params = self.parse_vec_valuetype()?;
        let result = self.parse_vec_valuetype()?;

        Ok(function::FuncType::new(Some(params), Some(result)))
    }

    pub fn parse_limit(&mut self) -> ParseResult<memory::Limit> {
        if let Some(0x00) = self.lookahead {
            self.next_byte();

            let min = self.scan_u32()?;
            let max = None;

            Ok(memory::Limit::new(min, max))
        } else if let Some(0x01) = self.lookahead {
            self.next_byte();

            let min = self.scan_u32()?;
            let max = self.scan_u32()?;

            Ok(memory::Limit::new(min, Some(max)))
        } else {
            Err(Error::InvalidLimit)
        }
    }

    pub fn parse_table_type(&mut self) -> ParseResult<memory::TableType> {
        let elemtype = get_elemtype(self.next_byte()?)?;
        let limit = self.parse_limit()?;

        Ok(memory::TableType::new(elemtype, limit))
    }

    pub fn parse_global_type(&mut self) -> ParseResult<types::GlobalType> {
        let valuetype = self.parse_valuetype()?;
        let vis = get_mut_type(self.next_byte()?)?;

        Ok(types::GlobalType::new(vis, valuetype))
    }

    // pub fn next(&mut self) -> AstElem {

    // }
}

#[inline]
fn eof(byte: Option<u8>) -> ParseResult<u8> {
    byte.ok_or_else(|| Error::EOF)
}

pub fn get_valtype(code: u8) -> ParseResult<ValueType> {
    match code {
        0x7F => Ok(ValueType::Int32),
        0x7E => Ok(ValueType::Int64),
        0x7D => Ok(ValueType::Float32),
        0x7C => Ok(ValueType::Float64),
        _ => Err(Error::InvalidValueType),
    }
}

pub fn get_elemtype(code: u8) -> ParseResult<memory::ElemType> {
    match code {
        0x70 => Ok(memory::ElemType::FuncRef),
        _ => Err(Error::InvalidElemType),
    }
}

pub fn get_mut_type(code: u8) -> ParseResult<types::Mut> {
    match code {
        0x00 => Ok(types::Mut::Const),
        0x01 => Ok(types::Mut::Var),
        _ => Err(Error::InvalidMutability),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valuetype() {
        let source = [0x7F];

        let mut parser = Parser::new(&source);

        println!("{:?}", parser.parse_valuetype());
    }
}
