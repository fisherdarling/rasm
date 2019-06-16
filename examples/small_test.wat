(module
    ;; (func $test (result i32)
    ;;    i32.const 1
    ;;    (if (result i32)
    ;;        (then
    ;;            i32.const 5)
    ;;        (else
    ;;            i32.const 10)))

    (func $baz (export "baz")
          (loop
            br 0))

    (func $buzz (export "buzz") (result i32)
        (block (result i32)
          i32.const 42
          (block (result i32)
            i32.const 10
            br 1)
          br 0))

    (func $fizz (export "fizz") (result i32)
        i32.const 1
        (if (result i32)
          (then
            i32.const 0
            (if (result i32)
              (then
                i32.const 1)
              (else
                i32.const 2)))
          (else
            i32.const 10)))
    ;;(func $buzz
    ;;    (loop $L0
    ;;          br $L0))
)

