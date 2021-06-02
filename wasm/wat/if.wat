(module
  (func $if (param $lhs i32) (result i32) (local i32)
    get_local $lhs
    if
      i32.const 100
      set_local 0
    else
      i32.const 1
      set_local 0
    end
    get_local 0)
  (export "if" (func $if))
)
