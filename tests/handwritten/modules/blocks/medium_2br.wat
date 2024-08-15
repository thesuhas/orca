(module
  (type (;0;) (func))
  (func (;1;) (type 0)
    block $hi
        block
            br $hi
            i32.const 1
            i32.const 2
            i32.add
            drop
            br $hi
        end
    end
  )
  (memory (;0;) 1)
)
