(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32 i32 i32 i32 i32 i32)))
  (type (;5;) (func (param i32 i32) (result i32)))
  (import "bogus" "hi" (func (;0;) (type 2)))
  (import "orca" "better" (func (;1;) (type 2))) ;; rm
  (import "than" "walrus" (func (;2;) (type 2))) ;; rm
  (func (;3;) (type 0) ;; rm
    i32.const 0 ;; rm
    drop ;; rm
    ) ;; rm
  (func $add (;4;) (type 5) (param i32 i32) (result i32)
    (local f32)
    local.get 0
    local.get 1
    i32.add)
  (func (;5;) (type 0)
    (local i32)
    i32.const 1
    i32.const 2
    call 4 ;; < call 1
    drop
    )
   ;; << (func (;5;) (type 0)
   ;; <<   i32.const 1
   ;; <<   drop
   ;; <<   )
  (memory (;0;) 1)
  (export "add" (func 4)) ;; < (export "add" (func 1))
  (export "memory" (memory 0))
)