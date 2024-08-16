(module
  (type (;0;) (func))
  (func (;0;) (type 0)
    ;; << (local i32 i32)
    block ;; label = @1
      i32.const 0
      ;; << i32.const 1
      ;; << local.set 0
      br_if 0 (;@1;)
      ;; << i32.const 0
      ;; << local.set 0
      ;; << i32.const 1234
      ;; << drop
      i32.const 0
      ;; << i32.const 1
      ;; << local.set 1
      br_if 0 (;@1;)
      ;; << i32.const 0
      ;; << local.set 1
      ;; << i32.const 1234
      ;; << drop
    end
    ;; << local.get 0
    ;; << if ;; label = @1
    ;; <<   i32.const 1234
    ;; <<   drop
    ;; << else
    ;; <<   local.get 1
    ;; <<   if ;; label = @2
    ;; <<     i32.const 1234
    ;; <<     drop
    ;; <<   end
    ;; << end
  )
  (memory (;0;) 1)
)
