#![feature(fixed_size_array)]
use std::array::FixedSizeArray;

use wabi::runtime::ModuleInstance;
use wabi::types::Value;

fn main() {
    let input = include_bytes!("../../../wasm-nom/examples/add.wasm");

    let _ = env_logger::try_init().unwrap();

    let mut runtime = ModuleInstance::from_bytes(input.as_slice()).unwrap();

    let args = vec![Value::I32(5), Value::I32(5)];

    let res = runtime.invoke("add", &args).unwrap();

    println!("Function: {:?}, Args: {:?}", "add", args);
    println!("Result: {:?}", res);
}
