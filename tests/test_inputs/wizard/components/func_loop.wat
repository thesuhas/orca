(component
  (core module (;0;)
    (type (;0;) (func))
    (func (;0;) (type 0)
      (local i32)
      i32.const 5
      local.set 0
      loop ;; label = @1
        local.get 0
        i32.const 1
        i32.sub
        local.set 0
        local.get 0
        br_if 0 (;@1;)
      end
    )
    (func (;1;) (type 0)
      (local i32)
      i32.const 5
      local.set 0
      loop ;; label = @1
        local.get 0
        i32.const 1
        i32.sub
        local.set 0
        local.get 0
        br_if 0 (;@1;)
      end
    )
    (export "main" (func 0))
    (export "_start" (func 1))
  )
  (core instance (;0;) (instantiate 0))
  (@producers
    (processed-by "wit-component" "0.207.0")
  )
)
