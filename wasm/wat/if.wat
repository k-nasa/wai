(module
  (func $if (param $lhs i32) (result i32) (local i32)
    i32.const 1
    set_local 0

    get_local $lhs
    if
      get_local 0
      i32.const 1
      i32.add
      set_local 0
    end
    get_local 0)
  (export "if" (func $if))
)
