#![feature(fixed_size_array)]
use std::array::FixedSizeArray;

use yawr::runtime::Runtime;
use yawr::types::Value;

use std::time::{Duration, Instant};

fn main() {
    let input = include_bytes!("../../../examples/fib_bench.wasm");

    let _ = env_logger::try_init().unwrap();

    let mut runtime = Runtime::from_bytes(input.as_slice()).unwrap();

    let args = vec![Value::I32(10)];

    // let res = runtime.invoke("fib", &args).unwrap();

    println!("Executing: {}({:?})", "fib", args[0]);

    println!("Warming up with 5 runs...");
    let start = Instant::now();

    for _ in 0..5 {
        runtime.invoke("fib", &args).unwrap();
    }

    println!("Warmup took {:?}", start.elapsed());
    println!("Running 50 invocations");

    let mut total: Duration = Default::default();

    for _ in 0..50 {
        let start = Instant::now();
        runtime.invoke("fib", &args).unwrap();
        total += start.elapsed();
    }

    println!("[average] {:?} [total] {:?}", total / 50, total);

    let res = runtime.invoke("fib", &args).unwrap();
    println!("Result: {:?}", res);
}
