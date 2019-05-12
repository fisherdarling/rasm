use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Keyword(&'a str),
    Int(&'a str),
    Float(&'a str),
    Str(&'a str),
    Id(&'a str),
    Lparen,
    Rparen,
    Reserved(&'a str),
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Keyword(k) => write!(f, "{}", k),
            Token::Int(i) => write!(f, "{}", i),
            Token::Float(fl) => write!(f, "{}", fl),
            Token::Str(s) => write!(f, "{}", s),
            Token::Id(id) => write!(f, "{}", id),
            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
            Token::Reserved(r) => write!(f, "{}", r),
        }
    }
}
