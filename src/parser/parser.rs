use crate::lexer::{Lexer, Token};
use crate::parser::AstElem;
use crate::types::*;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer }
    }

    pub fn Parser(&mut self) -> AstElem {
        AstElem::Module(module::Module::new())
    }
}
