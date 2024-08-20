(module
  (type (;0;) (func))
  (func (;0;) (type 0)
    block $hi
      br $hi
      i32.const 1
      i32.const 2
      i32.add
      drop
      ;; << i32.const 12
      ;; << drop
    end
  )
  (memory (;0;) 1)
)
