#[macro_export]
macro_rules! test_file {
    ($name:ident, $path:literal, $func:ident ( $($args:literal),* ) => $expected:literal) => {
        #[test]
        fn $name() {
            let args = yawr::args!($($args),*);
            let func_name = stringify!($func);
            let expected_value = WasmResult::from(Value::from($expected));

            let mut runtime = ModuleInstance::from_file($path).expect(&format!("Unable to create runtime from `{}` for test: `{}`", $path, stringify!($name)));

            let result = runtime.invoke(func_name, &args).expect(&format!("Error executing `{}` for test: {}", func_name, stringify!($name)));

            assert_eq!(expected_value, result)
        }
    };
}

use yawr::runtime::ModuleInstance;
use yawr::types::{Value, WasmResult};

test_file!(add, "../examples/add.wasm", add(1_i32, 1_i32) => 2_i32);
test_file!(factorial, "../examples/fact.wasm", factorial(10_i64) => 3628800_i64);
test_file!(fib_bench, "../examples/fib_bench.wasm", fib(10_i32) => 55_i32);
test_file!(mem_check, "../examples/mem_check.wasm", mem_check(0_i32) => 30_i32);
test_file!(small_test_fizz, "../examples/small_test.wasm", fizz() => 2_i32);
test_file!(sum_easy, "../examples/sum_easy.wasm", sum(10_i32) => 55_i32);
test_file!(sum_hard, "../examples/sum_hard.wasm", sum(10_i32) => 55_i32);
