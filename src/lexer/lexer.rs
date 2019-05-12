use std::str::CharIndices;

pub struct Lexer<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
    lookahead: Option<char>,
    pos: usize,
    line_number: usize,
}

pub enum Token<'a> {
    Any(&'a str),
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
        match self.lookahead {
            Some('"') => true,
            _ => false,
        }
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

    // fn scan_comment(&mut self) -> Token<'a> {
    //     // Consume the ';'
    //     self.next_char();

    //     let text = self.rest_of_line();
    //     Token::Comment(text)
    // }

    fn looking_at_numeric(&self) -> bool {
        if let Some(c) = self.lookahead {
            if c.is_digit(10) {
                return true;
            }
            match c {
                '-' | '+' | '.' => return true,
                _ => {}
            }
        }

        false
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
                    ' ' | '\'' | '"' | ',' | ';' | '}' | '{' | '(' | '(' | '[' | ']' => false,
                    _ => true,
                }
            };
        }

        false
    }

    fn looking_at_stringchar(&self) -> bool {
        if let Some(c) = self.lookahead {
            if c >= '\u{20}' && c != '\u{7F}' && c != '\"' {
                return true;
            }

            match c {
                '\t' |
                '\n' |
                '\r' |
                '\'' |
                '\\' => return true,
                _ => return false,
            }
        }

        false
    }
    // fn scan_string(&mut self) -> Token<'a> {
    //     let start = self.pos;

    //     assert!(self.looking_at_str());

    //     loop {
    //         match self.next_char() {
    //             Some('"') => break,
    //             Some(_) if self.looking_at("\\\"") => {
    //                 // We're looking at a \, and know that the next
    //                 // char is a ", so consume both, the " will be
    //                 // consumed on the next iteration;
    //                 assert_eq!(self.next_char(), Some('"'));
    //             }
    //             None => panic!("Malformed string"),
    //             _ => {}
    //         }
    //     }

    //     // Consume the end quote
    //     self.next_char();

    //     let pre_text = &self.source[start + 1..self.pos - 1];

    //     Token::StrLit(pre_text)
    // }
    // fn scan_number(&mut self) -> Token<'a> {
    //     let start = self.pos;
    //     let mut is_float = false;

    //     match self.lookahead {
    //         Some('-') => {
    //             self.next_char();
    //             if !self.looking_at_numeric() {
    //                 return Token::Symbol("-");
    //             }
    //         }
    //         Some('+') => {
    //             self.next_char();
    //             if !self.looking_at_numeric() {
    //                 return Token::Symbol("+");
    //             }
    //         }
    //         _ => {}
    //     }

    //     if let Some('.') = self.lookahead {
    //         is_float = true;

    //         match self.next_char() {
    //             Some(c) if c.is_numeric() => {}
    //             _ => panic!(),
    //         }
    //     }

    //     loop {
    //         match self.next_char() {
    //             Some('.') => is_float = true,
    //             Some(ch) if ch.is_numeric() => {}
    //             _ => break,
    //         }
    //     }

    //     let text = &self.source[start..self.pos];

    //     if is_float {
    //         Token::Float(text)
    //     } else {
    //         Token::Int(text)
    //     }
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello() {
        // let source = ""

        // let mut lexer =
    }
}
