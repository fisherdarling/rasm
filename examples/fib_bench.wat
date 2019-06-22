(module
  (type $t0 (func (param i32) (result i32)))
  (func $fib (export "fib") (type $t0) (param $p0 i32) (result i32)
    block $B0
      block $B1
        get_local $p0
        i32.eqz
        br_if $B1
        get_local $p0
        i32.const 1
        i32.ne
        br_if $B0
        i32.const 1
        return
      end
      i32.const 0
      return
    end
    get_local $p0
    i32.const -1
    i32.add
    call $fib
    get_local $p0
    i32.const -2
    i32.add
    call $fib
    i32.add)
  (table $T0 1 1 anyfunc)
  (memory $memory (export "memory") 17))

