use wabt::Wat2Wasm;

use yawr::args;
use yawr::runtime::ModuleInstance;


use env_logger::try_init;

fn main() {
    let _ = try_init().unwrap();

    let wasm_binary = Wat2Wasm::new()
        .convert(
            r#"
        (module
            (func $dummy)
            
            
            (func (export "as-compare-operand") (result i32)
    (i32.gt_u
      (global.get 0) (i32.const 1)
    )
  )

        (global $a i32 (i32.const -2))
  (global (;1;) f32 (f32.const -3))
  (global (;2;) f64 (f64.const -4))
  (global $b i64 (i64.const -5))

  (global $x (mut i32) (i32.const -12))
  (global (;5;) (mut f32) (f32.const -13))
  (global (;6;) (mut f64) (f64.const -14))
  (global $y (mut i64) (i64.const -15))

        )
    "#,
        )
        .unwrap()
        .as_ref()
        .to_vec();

    let mut runtime = ModuleInstance::from_bytes(wasm_binary).unwrap();

    let func = "as-compare-operand";
    let args = args![];

    println!("[running]: {}({:?}):", func, args);

    let res = runtime.invoke(func, args);

    println!("[result]:  {:?}", res);
}
