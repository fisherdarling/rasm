#![feature(fixed_size_array)]
use std::array::FixedSizeArray;

use yawr::runtime::Runtime;
use yawr::types::Value;

use std::time::{Duration, Instant};

use criterion::*;

// fn main() {
//     let input = include_bytes!("../../../examples/fib_bench.wasm");
//     let mut runtime = Runtime::from_bytes(input);

//     let _ = env_logger::try_init().unwrap();

//     let args = vec![Value::I32(10)];

//     // let res = runtime.invoke("fib", &args).unwrap();

//     println!("Executing: {}({:?})", "fib", args[0]);

//     println!("Warming up with 5 runs...");
//     let start = Instant::now();

//     for _ in 0..5 {
//         runtime.invoke("fib", &args).unwrap();
//     }

//     println!("Warmup took {:?}", start.elapsed());
//     println!("Running 50 invocations");

//     let mut total: Duration = Default::default();

//     for _ in 0..50 {
//         let start = Instant::now();
//         runtime.invoke("fib", &args).unwrap();
//         total += start.elapsed();
//     }

//     println!("[average] {:?} [total] {:?}", total / 50, total);

//     // let res =
//     println!("Result: {:?}", res);
// }

fn fibonacci_name(c: &mut Criterion) {
    let bytes = include_bytes!("../../examples/fib_bench.wasm");
    let mut runtime = Runtime::from_bytes(bytes.as_slice());

    let args = vec![Value::I32(10)];

    c.bench_function("fib_name", move |b| b.iter(|| runtime.invoke("fib", &args)));
}

fn fibonacci_index(c: &mut Criterion) {
    let bytes = include_bytes!("../../examples/fib_bench.wasm");
    let mut runtime = Runtime::from_bytes(bytes.as_slice());

    let args = vec![Value::I32(10)];

    c.bench_function("fib_index", move |b| b.iter(|| runtime.invoke_index(0, &args)));
}

criterion_group!(benches, fibonacci_name, fibonacci_index);
criterion_main!(benches);
