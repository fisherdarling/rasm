use std::fs::read_to_string;
use std::str::CharIndices;

use rasm::lexer::Lexer;

fn main() {
    let code = read_to_string("./examples/simple_add.wat").unwrap();
    let mut lexer = Lexer::new(&code);

    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        tokens.push(token);
    }

    println!("{:?}", tokens);
}
