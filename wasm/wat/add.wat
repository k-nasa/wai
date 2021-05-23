(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (func $add (type 0) (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.add)
  (export "add" (func $add)))
