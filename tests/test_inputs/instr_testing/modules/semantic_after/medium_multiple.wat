(module
  (type (;0;) (func))
  (func (;0;) (type 0)
    ;; << (local i32 i32)
    block $hi
      i32.const 1
      ;; << i32.const 1
      ;; << local.set 0
      br_if $hi
      ;; << i32.const 0
      ;; << local.set 0
      ;; << i32.const 1234
      ;; << drop
      block ;; label = @2
        i32.const 1
        ;; << i32.const 1
        ;; << local.set 1
        br_table 0 (;@2;) $hi
        ;; << i32.const 0
        ;; << local.set 1
      end
      ;; << local.get 1
      ;; << if ;; label = @2
      ;; <<   i32.const 5678
      ;; <<   drop
      ;; << end
    end
    ;; << local.get 0
    ;; << if ;; label = @1
    ;; <<   i32.const 1234
    ;; <<   drop
    ;; << else
    ;; <<   local.get 1
    ;; <<   if ;; label = @2
    ;; <<     i32.const 5678
    ;; <<     drop
    ;; <<   end
    ;; << end
  )
  (memory (;0;) 1)
)
