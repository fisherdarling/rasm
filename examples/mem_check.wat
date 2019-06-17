(module
  (type $t0 (func (param i32) (result i32)))
  (func $mem_check (export "mem_check") (type $t0) (param $p0 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32)
    i32.const -16
    set_local $l0
    loop $L0
      get_local $l0
      i32.const 1040
      i32.add
      tee_local $l1
      get_local $l1
      i32.load
      tee_local $l1
      get_local $l1
      i32.mul
      i32.store
      get_local $l0
      i32.const 4
      i32.add
      tee_local $l0
      br_if $L0
    end
    i32.const 0
    set_local $l1
    i32.const -16
    set_local $l0
    loop $L1
      get_local $l0
      i32.const 1040
      i32.add
      i32.load
      get_local $l1
      i32.add
      set_local $l1
      get_local $l0
      i32.const 4
      i32.add
      tee_local $l2
      set_local $l0
      get_local $l2
      br_if $L1
    end
    get_local $l1)
  (table $T0 1 1 anyfunc)
  (memory $memory (export "memory") 17)
  (data (i32.const 1024) "\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00"))
