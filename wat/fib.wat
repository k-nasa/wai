(module

  ;; recursive implementation

  (func $fib_recursive (param $N i32) (result i32)
    (if
      (i32.eq (get_local $N) (i32.const 1))
      (then (return (i32.const 1)))
    )
    (if
      (i32.eq (get_local $N) (i32.const 2))
      (then (return (i32.const 1)))
    )
    (i32.add (call $fib_recursive
      (i32.sub (get_local $N) (i32.const 1))
      (i32.sub (get_local $N) (i32.const 2))
    ))
  )

  ;; iterative implementation, avoids stack overflow

  (func $fib_iterative (param $N i32) (result i32)
    (local $n1 i32)
    (local $n2 i32)
    (local $tmp i32)
    (local $i i32)
    (set_local $n1 (i32.const 1))
    (set_local $n2 (i32.const 1))
    (set_local $i (i32.const 2))


    ;; return 0 for N <= 0
    (if
      (i32.le_s (get_local $N) (i32.const 0))
      (then (return (i32.const 0)))
    )

    ;;since we normally return n2, handle n=1 case specially
    (if
      (i32.le_s (get_local $N) (i32.const 2))
      (then (return (i32.const 1)))
    )

    (loop $again
      (if
        (i32.lt_s (get_local $i) (get_local $N))
        (then
          (set_local $tmp (i32.add (get_local $n1) (get_local $n2)))
          (set_local $n1 (get_local $n2))
          (set_local $n2 (get_local $tmp))
          (set_local $i (i32.add (get_local $i) (i32.const 1)))
          (br $again)
        )
      )
    )

    (get_local $n2)
  )

  ;; export fib_iterative as the main thing, because it's the fastest

  (export "fib" (func $fib_iterative))
  (export "fib_iterative" (func $fib_iterative))
  (export "fib_recursive" (func $fib_recursive))
)

(assert_return (invoke "fib" (i32.const 10)) (i32.const 55))
(assert_return (invoke "fib_recursive" (i32.const 10)) (i32.const 55))
