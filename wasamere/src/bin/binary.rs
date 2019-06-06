// use std::fs::;

use env_logger::try_init;
use wasamere::module::ParsedModule;
use wasamere::StructNom;

fn main() {
    let _ = try_init().unwrap();

    let source = include_bytes!("../../examples/add.wasm");

    let module = ParsedModule::nom(source);

    println!("{:#?}", module);
}
