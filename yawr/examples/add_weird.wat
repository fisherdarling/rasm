(module
  (type $t0 (func (param i32) (result i32)))
  (func $weird_add (export "weird_add") (type $t0) (param $p0 i32) (result i32)
    i32.const 3
    i32.const -1
    get_local $p0
    i32.const 5
    i32.gt_s
    select
    get_local $p0
    i32.add)
  (table $T0 1 1 anyfunc)
  (memory $memory (export "memory") 17))
