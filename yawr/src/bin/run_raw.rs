use wabt::Wat2Wasm;

use yawr::runtime::Runtime;
use yawr::types::Value;
use yawr::args;


use env_logger::try_init;

fn main() {
    let _ = try_init().unwrap();

    let wasm_binary = Wat2Wasm::new()
        .convert(r#"
        (module
            (func $dummy)
            
            
            (func (export "loop2") (result i32)
    (local $i i32)
    (local.set $i (i32.const 0))
    (block $exit (result i32)
      (loop $cont (result i32)
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (if (i32.eq (local.get $i) (i32.const 5))
          (then (br $cont))
        )
        (if (i32.eq (local.get $i) (i32.const 8))
          (then (br $exit (local.get $i)))
        )
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $cont)
      )
    )
  )

        )
    "#).unwrap().as_ref().to_vec();

    let mut runtime = Runtime::from_bytes(wasm_binary).unwrap();

    let func = "loop2";
    let args = args![];

    println!("[running]: {}({:?}):", func, args);
    
    let res = runtime.invoke(func, args);
    
    println!("[result]:  {:?}", res);
}



