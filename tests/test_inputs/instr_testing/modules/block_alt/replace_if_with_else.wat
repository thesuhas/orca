(module
  (type (;0;) (func))
  (func (;0;) (type 0)
    i32.const 0
    ;; << drop
    ;; << i32.const 12
    ;; << drop
    if ;; rm
      i32.const 0 ;; rm
      drop ;; rm
    else ;; rm
      i32.const 1 ;; rm
      drop ;; rm
    end ;; rm
  )
  (memory (;0;) 1)
)
