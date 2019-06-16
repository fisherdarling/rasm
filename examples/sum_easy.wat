(module
  (type $t0 (func (param i32) (result i32)))
  (func $sum (export "sum") (type $t0) (param $p0 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32)
    i32.const 0
    set_local $l0
    block $B0
      get_local $p0
      i32.const 0
      i32.lt_s
      br_if $B0
      i32.const 0
      set_local $l0
      i32.const 0
      set_local $l1
      loop $L1
        get_local $l0
        get_local $l1
        i32.add
        set_local $l0
        get_local $l1
        i32.const 1
        i32.add
        i32.const 1
        get_local $l1
        get_local $p0
        i32.lt_s
        tee_local $l2
        select
        tee_local $l3
        set_local $l1
        get_local $l3
        get_local $p0
        i32.const 0
        get_local $l2
        select
        tee_local $p0
        i32.le_s
        br_if $L1
      end
    end
    get_local $l0)
  (table $T0 1 1 anyfunc)
  (memory $memory (export "memory") 17))

