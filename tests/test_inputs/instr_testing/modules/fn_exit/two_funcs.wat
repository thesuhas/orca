(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32 i32 i32 i32 i32 i32)))
  (type (;5;) (func (param i32 i32) (result i32)))
  (import "bogus" "hi" (func (;0;) (type 2)))
  (func $add (;1;) (type 5) (param i32 i32) (result i32)
    (local f32)
    ;; << block (type 1) (result i32) ;; label = @1
    local.get 0
    local.get 1
    i32.add
    ;; << end
    ;; << i32.const 1
    ;; << drop
  )
  (func (;2;) (type 0)
    (local i32)
    ;; << block (type 0)
    i32.const 1
    i32.const 2
    call $add
    drop
    ;; << end
    ;; << i32.const 1
    ;; << drop
  )
  (memory (;0;) 1)
  (export "add" (func $add))
  (export "memory" (memory 0))
)
