(module
  (type (;0;) (func))
  (func (;0;) (type 0)
    ;; << (local i32)
    block ;; label = @1
      block ;; label = @2
        br_if 0 (;@2;)
        br_table 0 (;@2;)
        if ;; label = @3
          ;; << i32.const 1
          ;; << local.set 0
          br 0 (;@3;)
          ;; << i32.const 0
          ;; << local.set 0
        end
        ;; << local.get 0
        ;; << if ;; label = @3
        ;; <<   i32.const 34
        ;; <<   drop
        ;; << end
        ;; << i32.const 12
        ;; << drop
      end
    end
  )
  (memory (;0;) 1)
)
