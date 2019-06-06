// use std::fs::;

use env_logger::try_init;
// use wasamere::module::ParsedModule;

fn main() {
    let _ = try_init().unwrap();

    let source = include_bytes!("../../examples/add.wasm");

    // let module = ParsedModule::from_bytes(source);

    // println!("{:#?}", module);
}
