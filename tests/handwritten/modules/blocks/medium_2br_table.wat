(module
  (type (;0;) (func))
  (func (;1;) (type 0)
    block $hi
        block
            i32.const 1
            br_table 0 $hi
            i32.const 1
            i32.const 2
            i32.add
            drop
            i32.const 1
            br_table 0 $hi
            i32.const 1
            i32.const 2
            i32.add
            drop
        end
    end
  )
  (memory (;0;) 1)
)
