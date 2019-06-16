use yawr::runtime::Runtime;
use yawr::types::Value;

use std::time::{Instant, Duration};

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

fn fibonacci(c: &mut Criterion) {
    let bytes = include_bytes!("../../examples/fib_bench.wasm");
    let mut runtime = Runtime::from_bytes(bytes);
    
    let args = vec![Value::I32(10)];
    
    c.bench_function("fib 10", move |b| b.iter(|| runtime.invoke("fib", &args)));
}   

criterion_group!(benches, fibonacci);
criterion_main!(benches);