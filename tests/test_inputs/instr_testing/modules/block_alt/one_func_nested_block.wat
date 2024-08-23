(module
  (type (;0;) (func))
  (func (;0;) (type 0)
    block $hi
      ;; << i32.const 12
      ;; << drop
      loop ;; rm
        br $hi ;; rm
        i32.const 1 ;; rm
        i32.const 2 ;; rm
        i32.add ;; rm
        drop ;; rm
      end ;; rm
    end
  )
  (memory (;0;) 1)
)
