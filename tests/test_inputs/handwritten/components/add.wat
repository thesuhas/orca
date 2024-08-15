(component
  (core module (;0;)
    (type (;0;) (func))
    (type (;1;) (func (param i32 i32) (result i32)))
    (func $#func0<add> (@name "add") (;0;) (type 1) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      i32.add
    )
    (func (;1;) (type 0)
      i32.const 1
      i32.const 2
      call 1
      drop
      drop
    )
    (memory (;0;) 1)
    (export "add" (func 1))
    (export "memory" (memory 0))
  )
  (core instance (;0;) (instantiate 0))
  (alias core export 0 "memory" (core memory (;0;)))
  (@producers
    (processed-by "wit-component" "0.207.0")
  )
)
