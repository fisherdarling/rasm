fn main() {
    let source = include_bytes!("../../examples/large_func.wasm");

    let strings: Vec<_> = source.into_iter().map(|b| format!("0x{:2x?}", b)).collect();

    println!("{:?}", strings);
}
