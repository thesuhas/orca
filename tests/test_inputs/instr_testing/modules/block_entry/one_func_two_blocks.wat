(module
  (type (;0;) (func))
  (func (;0;) (type 0)
    block $hi
      ;; << i32.const 12
      ;; << drop
      i32.const 1
      if ;; label = @2
        ;; << i32.const 56
        ;; << drop
        br $hi
        i32.const 1
        i32.const 2
        i32.add
        drop
      end
    end
    block $hi
      ;; << i32.const 12
      ;; << drop
      i32.const 1
      drop
      block ;; label = @2
        ;; << i32.const 12
        ;; << drop
        br $hi
        i32.const 1
        i32.const 2
        i32.add
        drop
      end
    end
  )
  (memory (;0;) 1)
)
