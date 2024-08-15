(module
    (type (;0;) (func))
    (func (;1;) (type 0)
        block
            block
                i32.const 0
                if
                    i32.const 1
                    drop
                end
                i32.const 0
                if
                    i32.const 1
                    drop
                else
                    i32.const 0
                    br_if 2
                end
                block
                    block
                        i32.const 1
                        br_table 3 2 0
                        block
                            i32.const 1
                            br_table 4 1
                        end
                    end
                    loop
                        i32.const 0
                        br 0
                    end
                end
            end
        end
    )
    (memory (;0;) 1)
)
