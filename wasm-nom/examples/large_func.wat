(module
(func $getCell (param $x i32) (param $y i32) (result i32)
  (if (result i32)
    (block (result i32)
      (i32.and
        (call 5
          (i32.const 0)
          (i32.const 50)
          (get_local $x))
        (call 10
          (i32.const 0)
          (i32.const 50)
          (get_local $y))))
    (then
      (i32.load8_u
        (call 15
          (get_local $x)
          (get_local $y))))
    (else
      (i32.const 0))))
  (export "getCell" (func $getCell)))
