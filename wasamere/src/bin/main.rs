use std::fs::read_to_string;
use std::str::CharIndices;

use wasamere::parser::Parser;

fn main() {
    let source = include_bytes!("../../examples/add.wasm");

    let mut parser = Parser::new(source);
}
