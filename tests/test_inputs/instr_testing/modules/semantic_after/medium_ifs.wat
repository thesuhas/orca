(module
  (type (;0;) (func))
  (func (;1;) (type 0)
    block $hi
        block
            i32.const 1
            br_table 0 $hi
            i32.const 1
            if
                br 1
            end
        end
    end
  )
  (memory (;0;) 1)
)
