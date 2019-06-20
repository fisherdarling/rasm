use wabt::Wat2Wasm;

use yawr::runtime::Runtime;
use yawr::args;

use env_logger::try_init;

fn main() {
    let _ = try_init().unwrap();

    let wasm_binary = Wat2Wasm::new()
        .convert(r#"
        (module
            (func $dummy)
            
            
            (func (export "nested-block-value") (result i32)
                (i32.add
                    (i32.const 1)
                    (block (result i32)
                        (call $dummy)
                        (i32.add 
                            (i32.const 4) 
                            (br 0 (i32.const 8))
                        )
                    )
                )
            )

        )
    "#).unwrap().as_ref().to_vec();

    let mut runtime = Runtime::from_bytes(wasm_binary).unwrap();

    let func = "nested-block-value";
    let args = args![];

    println!("[running]: {}({:?}):", func, args);
    
    let res = runtime.invoke(func, args);
    
    println!("[result]:  {:?}", res);
}



