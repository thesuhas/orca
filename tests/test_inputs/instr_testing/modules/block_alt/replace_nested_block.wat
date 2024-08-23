(module
  (type (;0;) (func))
  (func (;0;) (type 0)
    ;; << i32.const 12
    ;; << drop
    block $hi ;; rm
      i32.const 1 ;; rm
      if ;; rm
        br $hi ;; rm
        i32.const 1 ;; rm
        i32.const 2 ;; rm
        i32.add ;; rm
        drop ;; rm
      end ;; rm
      i32.const 1 ;; rm
      i32.const 2 ;; rm
      i32.add ;; rm
      drop ;; rm
    end ;; rm
  )
  (memory (;0;) 1)
)
