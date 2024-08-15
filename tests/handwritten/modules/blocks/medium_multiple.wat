(module
  (type (;0;) (func))
  (func (;1;) (type 0)
    block $hi
        i32.const 1
        br_if 0
        block
            i32.const 1
            br_table 0 $hi
        end
    end
  )
  (memory (;0;) 1)
)
