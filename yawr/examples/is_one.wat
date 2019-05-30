(module
  (type $t0 (func (param i32) (result i32)))
  (func $is_one (export "is_one") (type $t0) (param $p0 i32) (result i32)
    get_local $p0
    i32.const 1
    i32.eq)
  (table $T0 1 1 anyfunc)
  (memory $memory (export "memory") 17))

