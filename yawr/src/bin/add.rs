use yawr::runtime::Runtime;
use yawr::types::Value;

fn main() {
    let input = include_bytes!("../../../wasamere/examples/add.wasm");

    // let module = Module::from_bytes(input);

    let mut runtime = Runtime::from_bytes(input);

    let args = vec![Value::I32(1), Value::I32(1)];

    let res = runtime.invoke("add".to_string(), args);

    println!("{:?}", res);
}
