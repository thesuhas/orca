(component
  (core module (;0;)
    (type (;0;) (func (param i32) (result i64)))
    (func (;0;) (type 0) (param i32) (result i64)
      (local i32 i64)
      local.get 0
      local.get 1
      i32.add
      i64.extend_i32_s
    )
  )
  (core instance (;0;) (instantiate 0))
  (@producers
    (processed-by "wit-component" "0.207.0")
  )
)
