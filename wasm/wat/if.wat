(module
  (func $if (param $lhs i32) (result i32) (local i32)
    get_local $lhs
    if
      i32.const 10
      set_local 0
    end
    i32.const 10)
  (export "if" (func $if))
)
