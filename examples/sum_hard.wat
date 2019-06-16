(module
  (type $t0 (func (param i32) (result i32)))
  (func $sum (export "sum") (type $t0) (param $p0 i32) (result i32)
    (local $l0 i32)
    i32.const 0
    set_local $l0
    block $B0
      get_local $p0
      i32.const 0
      i32.lt_s
      br_if $B0
      block $B1
        block $B2
          get_local $p0
          i32.eqz
          br_if $B2
          get_local $p0
          i32.const 1
          get_local $p0
          i32.const 1
          i32.gt_s
          select
          tee_local $p0
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
          set_local $l0
          br $B1
        end
        i32.const 0
        set_local $l0
        i32.const 0
        set_local $p0
      end
      get_local $p0
      get_local $l0
      i32.add
      set_local $l0
    end
    get_local $l0)
  (table $T0 1 1 anyfunc)
  (memory $memory (export "memory") 17))
