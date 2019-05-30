(module
  (type $t0 (func (param i32) (result i32)))
  (func $sum_of_natural (export "sum_of_natural") (type $t0) (param $p0 i32) (result i32)
    (local $l0 i32)
    block $B0
      get_local $p0
      i32.const 1
      i32.lt_s
      br_if $B0
      get_local $p0
      i32.const -1
      i32.add
      tee_local $l0
      get_local $l0
      i64.extend_u/i32
      get_local $p0
      i32.const -2
      i32.add
      i64.extend_u/i32
      i64.mul
      i64.const 1
      i64.shr_u
      i32.wrap/i64
      i32.add
      return
    end
    i32.const 0)
  (table $T0 1 1 anyfunc)
  (memory $memory (export "memory") 17))
