use std::str::CharIndices;

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
    lookahead: Option<char>,
    pos: usize,
    line_number: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        let mut lex = Self {
            source,
            chars: source.char_indices(),
            lookahead: None,
            pos: 0,
            line_number: 1,
        };

        lex.next_char();

        lex
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some('\n') = self.lookahead {
            self.line_number += 1;
        }

        match self.chars.next() {
            Some((idx, ch)) => {
                self.pos = idx;
                self.lookahead = Some(ch);
            }
            None => {
                self.pos = self.source.len();
                self.lookahead = None;
            }
        }

        self.lookahead
    }

    fn scan_char(&mut self, token: Token<'a>) -> Token<'a> {
        self.next_char();
        token
    }

    fn looking_at(&self, prefix: &'a str) -> bool {
        self.source[self.pos..].starts_with(prefix)
    }

    fn looking_at_str(&self) -> bool {
        self.lookahead == Some('"')
    }

    fn rest_of_line(&mut self) -> &'a str {
        let start = self.pos;

        loop {
            match self.next_char() {
                None | Some('\n') => return &self.source[start..self.pos],
                _ => {}
            }
        }
    }

    fn scan_comment(&mut self) {
        // Consume the ';;'
        self.next_char();
        self.next_char();

        // Consume the rest of the line
        let text = self.rest_of_line();
    }

    fn looking_at_numeric(&self) -> bool {
        if self.looking_at("0x") {
            return true;
        }

        if let Some(c) = self.lookahead {
            if c.is_digit(10) {
                return true;
            }
            match c {
                '-' | '+' => return true,
                _ => {}
            }
        }

        false
    }

    fn scan_number(&mut self) -> Token<'a> {
        let start = self.pos;
        let mut is_float = false;
        let mut is_hex = false;

        // Handle signs
        match self.lookahead {
            Some('-') => {
                self.next_char();
                if !self.looking_at_numeric() {
                    return Token::Reserved("-");
                }
            }
            Some('+') => {
                self.next_char();
                if !self.looking_at_numeric() {
                    return Token::Reserved("+");
                }
            }
            _ => {}
        }

        if self.looking_at("0x") {
            is_hex = true;

            self.next_char();
            self.next_char();
        }

        if let Some('.') = self.lookahead {
            panic!();
        }

        loop {
            match self.next_char() {
                Some('.') => is_float = true,
                Some(ch) if is_hex && ch.is_digit(16) => {}
                Some(ch) if !is_hex && ch.is_digit(10) => {}
                Some('_') => {}
                _ => break,
            }
        }

        let text = &self.source[start..self.pos];

        if is_float {
            Token::Float(text)
        } else {
            Token::Int(text)
        }
    }

    fn looking_at_id(&self) -> bool {
        self.lookahead == Some('$')
    }

    fn looking_at_idchar(&self) -> bool {
        if let Some(c) = self.lookahead {
            return if !c.is_ascii_graphic() {
                false
            } else if c.is_digit(10) {
                true
            } else if c.is_ascii_alphabetic() {
                true
            } else {
                match c {
                    // Blacklist:
                    ' ' | '\'' | '"' | ',' | ';' | '}' | '{' | '(' | ')' | '[' | ']' => false,
                    _ => true,
                }
            };
        }

        false
    }

    fn scan_id(&mut self) -> Token<'a> {
        let start = self.pos;

        if !self.looking_at_idchar() {
            panic!("Ids must have at least one character");
        }

        while self.looking_at_idchar() {
            self.next_char();
        }

        let text = &self.source[start..self.pos];

        Token::Id(text)
    }

    fn scan_keyword(&mut self) -> Token<'a> {
        let start = self.pos;

        while self.looking_at_idchar() {
            self.next_char();
        }

        let text = &self.source[start..self.pos];

        Token::Keyword(text)
    }

    fn looking_at_stringchar(&self) -> bool {
        if let Some(c) = self.lookahead {
            if c >= '\u{20}' && c != '\u{7F}' && c != '\"' {
                return true;
            }

            match c {
                '\t' | '\n' | '\r' | '\'' | '\\' => return true,
                _ => return false,
            }
        }

        false
    }

    fn scan_str(&mut self) -> Token<'a> {
        let start = self.pos;

        assert!(self.looking_at_str());

        loop {
            match self.next_char() {
                None => panic!("Malformed string"),
                Some('"') => break,
                Some(_) if self.looking_at_stringchar() => {}
                Some(_) if self.looking_at("\\\"") => {
                    // We're looking at a \, and know that the next
                    // char is a ", so consume both, the " will be
                    // consumed on the next iteration;
                    assert_eq!(self.next_char(), Some('"'));
                }
                _ => {}
            }
        }

        // Consume the end quote
        self.next_char();

        let text = &self.source[start + 1..self.pos - 1];

        Token::Str(text)
    }

    pub fn next(&mut self) -> Option<Token<'a>> {
        loop {
            // println!(
            //     "[{:?}] [{:?}] [{:?}]",
            //     self.lookahead, self.pos, self.line_number
            // );

            return match self.lookahead {
                None => None,
                Some(c) if c.is_whitespace() => {
                    self.next_char();
                    continue;
                }
                Some(_) if self.looking_at(";;") => {
                    self.scan_comment();
                    continue;
                }
                Some('(') => Some(self.scan_char(Token::Lparen)),
                Some(')') => Some(self.scan_char(Token::Rparen)),
                Some('$') => Some(self.scan_id()),
                Some('"') => Some(self.scan_str()),
                Some(_) if self.looking_at_numeric() => Some(self.scan_number()),
                Some(_) if self.looking_at_idchar() => Some(self.scan_keyword()),
                c => unimplemented!("{:?}", c),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_proper() {
        let source = "$myid";

        let mut lexer = Lexer::new(source);

        let next = lexer.next().unwrap();

        assert_eq!(Token::Id("$myid"), next);
    }

    #[test]
    fn simple_int() {
        let source = "100_000_000";

        let mut lexer = Lexer::new(source);

        let next = lexer.next().unwrap();

        assert_eq!(Token::Int("100_000_000"), next);
    }

    #[test]
    fn simple_string() {
        let source = r#" "this is a string" "#;

        let mut lexer = Lexer::new(source);

        let next = lexer.next().unwrap();

        assert_eq!(Token::Str("this is a string"), next);
    }
}
