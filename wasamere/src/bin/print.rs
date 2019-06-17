fn main() {
    let source = include_bytes!("../../../examples/mem_check.wasm");

    let strings: Vec<_> = source.into_iter().map(|b| format!("{}", b)).collect();

    println!("{:#?}", strings);
}
