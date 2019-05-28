use yawr::module::Module;

fn main() {
    let input = include_bytes!("../../../wasamere/examples/add.wasm");

    let module = Module::from_bytes(input);

    println!("{:?}", module);
}
