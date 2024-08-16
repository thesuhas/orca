(module
  (type (;0;) (func))
  (func $main (type 0)
    (local $cnt i32)
    i32.const 5
    local.set $cnt
    loop  ;; label = @1
      local.get $cnt
      i32.const 1
      i32.sub
      local.set $cnt
      local.get $cnt
      br_if 0 (;@1;)
    end)
  (func $start (type 0)
    (local $cnt i32)
    i32.const 5
    local.set $cnt
    loop  ;; label = @1
      local.get $cnt
      i32.const 1
      i32.sub
      local.set $cnt
      local.get $cnt
      br_if 0 (;@1;)
    end)
  (export "main" (func $main))
  (export "_start" (func $start)))