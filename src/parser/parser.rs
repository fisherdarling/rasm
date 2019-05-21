use crate::lexer::{Lexer, Token};
use crate::types::*;
use crate::parser::AstElem;

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
