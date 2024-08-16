(module
  (type (;0;) (func))
  (func (;0;) (type 0)
    ;; << (local i32)
    block ;; label = @1
      i32.const 0
      ;; << i32.const 1
      ;; << local.set 0
      br_table 0 (;@1;)
      ;; << i32.const 0
      ;; << local.set 0
    end
    ;; << local.get 0
    ;; << if ;; label = @1
    ;; <<   i32.const 1234
    ;; <<   drop
    ;; << end
  )
  (memory (;0;) 1)
)
