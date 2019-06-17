use yawr::runtime::Runtime;
use yawr::types::Value;

fn main() {
    let bytes = include_bytes!("../../../examples/fib_bench.wasm");
    let mut runtime = Runtime::from_bytes(bytes);

    let args = vec![Value::I32(10)];

    let res = runtime.invoke("fib", &args);
    println!("Result: {:?}", res);
}
