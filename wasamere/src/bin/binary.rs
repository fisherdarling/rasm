// use std::fs::;

use wasamere::module::Module;
use env_logger::try_init;

fn main() {
    let _ = try_init().unwrap();

    let source = include_bytes!("../../examples/gol.wasm");

    let module = Module::from_bytes(source);

    println!("{:?}", module);
}
