use yawr::runtime::Runtime;
use yawr::types::Value;

fn main() {
    let input = include_bytes!("../../../wasamere/examples/add.wasm");

    let _ = env_logger::try_init().unwrap();

    let mut runtime = Runtime::from_bytes(input);

    let args = vec![Value::I32(5), Value::I32(5)];

    let res = runtime.invoke("add", &args);

    println!("Function: {:?}, Args: {:?}", "add", args);
    println!("Result: {:?}", res);
}
