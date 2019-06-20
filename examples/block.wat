(module
  (func $dummy)
  (func (;1;) (result i32)
    i32.const 1
    block (result i32)  ;; label = @1
      call $dummy
      i32.const 4
      i32.const 8
      br 0 (;@1;)
      i32.add
    end
    i32.add)
  (export "nested-block-value" (func 1))
  (type (;0;) (func))
  (type (;1;) (func (result i32))))
