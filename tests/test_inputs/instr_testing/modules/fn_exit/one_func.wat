(module
  (type (;0;) (func))
  (func (;0;) (type 0)
    block $hi ;; << (type 0)
      ;; << block
      block ;; label = @2
        br $hi ;; < br 1 (;@2;)
        i32.const 1
        i32.const 2
        i32.add
        drop
      end
      ;; << end
    end
    ;; << i32.const 1
    ;; << drop
  )
  (memory (;0;) 1)
)
