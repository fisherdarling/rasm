(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    get_local $lhs
    get_local $rhs
    i32.add)
  ;; (func $locals (param i32 i32) (result i32)
  ;;   (local i32 i32)
  ;;   (local f64)
  ;;   (local f32)
  ;;   get_local 0
  ;;   get_local 1
  ;;   i32.add)
  (export "add" (func $add))
)