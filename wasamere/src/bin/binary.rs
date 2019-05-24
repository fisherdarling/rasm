// use std::fs::;

use wasamere::module::Module;

fn main() {
    let source = include_bytes!("../../examples/call.wasm");

    let module = Module::from_bytes(source);

    println!("{:?}", module);
}
