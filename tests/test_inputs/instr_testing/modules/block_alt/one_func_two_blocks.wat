(module
  (type (;0;) (func))
  (func (;0;) (type 0)
    ;; << i32.const 12
    ;; << drop
    block $hi ;; rm
      br $hi ;; rm
    end ;; rm
    ;; << i32.const 12
    ;; << drop
    block $hi ;; rm
      br $hi ;; rm
    end ;; rm
  )
  (memory (;0;) 1)
)
