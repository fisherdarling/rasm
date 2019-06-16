(module
  (type $t0 (func (param i64) (result i64)))
  (func $factorial (export "factorial") (type $t0) (param $p0 i64) (result i64)
    i64.const 1
    get_local $p0
    i64.eq
    (if (result i64)
      (then
        i64.const 1)
      (else
        get_local $p0
        i64.const -1
        i64.add
        call $factorial
        get_local $p0
        i64.mul)))
  (table $T0 1 1 anyfunc)
  (memory $memory (export "memory") 17))
