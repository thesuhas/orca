(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32 i32 i32 i32 i32 i32)))
  (type (;5;) (func (param i32 i32) (result i32)))
  ;; << (type (;6;) (func))
  ;; << (type (;7;) (func))
  (import "bogus" "hi" (func (;0;) (type 2)))
  ;; << (import "test0" "func0" (func (;1;) (type 2)))
  ;; << (import "test1" "func1" (func (;2;) (type 2)))
  (func (;1;) (type 0) ;; < (func (;3;) (type 0)
    i32.const 0
    i32.const 1
    call 0
    )
  (func (;2;) (type 5) (param i32 i32) (result i32) ;; < (func (;4;) (type 5) (param i32 i32) (result i32)
    (local f32)
    local.get 0
    local.get 1
    i32.add)
  (func (;3;) (type 0) ;; < (func (;5;) (type 0)
    (local i32)
    i32.const 1
    i32.const 2
    call 2 ;; < call 4
    drop
    )
  ;; << (func (;6;) (type 6)
  ;; <<   i32.const 1
  ;; <<   i32.const 1
  ;; <<   call 1
  ;; << )
  ;; << (func (;7;) (type 7)
  ;; <<   i32.const 2
  ;; <<   drop
  ;; <<   call 6
  ;; << )
  (memory (;0;) 1)
  (export "add" (func 2)) ;; < (export "add" (func 4))
  (export "memory" (memory 0))
)
