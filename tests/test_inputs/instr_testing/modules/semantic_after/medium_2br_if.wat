(module
  (type (;0;) (func))
  (func (;1;) (type 0)
    block $hi
        block
            i32.const 0
            br_if $hi
            i32.const 1
            i32.const 2
            i32.add
            drop
            i32.const 0
            br_if $hi
        end
    end
  )
  (memory (;0;) 1)
)
