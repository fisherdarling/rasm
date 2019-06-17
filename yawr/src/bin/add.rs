#![feature(fixed_size_array)]
use std::array::FixedSizeArray;

use yawr::runtime::Runtime;
use yawr::types::Value;

fn main() {
    let input = include_bytes!("../../../wasamere/examples/add.wasm");

    let _ = env_logger::try_init().unwrap();

    let mut runtime = Runtime::from_bytes(input.as_slice());

    let args = vec![Value::I32(5), Value::I32(5)];

    let res = runtime.invoke("add", &args).unwrap();

    println!("Function: {:?}, Args: {:?}", "add", args);
    println!("Result: {:?}", res);
}
