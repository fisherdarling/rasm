use yawr::runtime::Runtime;
use yawr::types::WasmValue;

fn main() {
    let input = include_bytes!("../../../wasamere/examples/add.wasm");

    // let module = Module::from_bytes(input);

    let mut runtime = Runtime::from_bytes(input);

    let args = vec![WasmValue::I32(1), WasmValue::I32(1)];

    let res = runtime.invoke("add".to_string(), args);

    println!("{:?}", res);
}
