#![feature(fixed_size_array)]
use std::array::FixedSizeArray;

use wabi::runtime::Runtime;
use wabi::types::Value;

use criterion::*;

// fn main() {
//     let input = include_bytes!("../../../examples/fib_bench.wasm");
//     let mut runtime = ModuleInstance::from_bytes(input);

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
    let vec: Vec<u8> = bytes.to_vec();

    let mut runtime = Runtime::default();
    runtime.add_module(None, &vec).unwrap();

    let args = vec![Value::I32(10)];

    c.bench_function("fib_name", move |b| b.iter(|| runtime.invoke("fib", &args)));
}

fn fibonacci_index(c: &mut Criterion) {
    let bytes = include_bytes!("../../examples/fib_bench.wasm");
    let vec: Vec<u8> = bytes.to_vec();

    let mut runtime = Runtime::default();
    runtime.add_module(None, &vec).unwrap();

    let args = vec![Value::I32(10)];

    c.bench_function("fib_index", move |b| {
        b.iter(|| runtime.invoke_index(0u32.into(), &args))
    });
}

criterion_group!(benches, fibonacci_name, fibonacci_index);
criterion_main!(benches);
