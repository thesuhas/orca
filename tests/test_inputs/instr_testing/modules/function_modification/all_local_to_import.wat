(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32 i32 i32 i32 i32 i32)))
  (type (;5;) (func (param i32 i32) (result i32)))
  (import "bogus" "hi" (func (;0;) (type 2)))
  (import "wirm" "better" (func (;1;) (type 2)))
  (import "than" "walrus" (func (;2;) (type 2)))
  ;; << (import "all" "local" (func (;0;) (type 2)))
  ;; << (import "to" "import" (func (;0;) (type 2)))
  ;; << (import "please" "work" (func (;0;) (type 2)))
  (func $local (;3;) (type 0) ;; rm
     i32.const 1 ;; rm
     drop ;; rm
     ) ;; rm
  (func $import (;4;) (type 0) ;; rm
    i32.const 2 ;; rm
    drop ;; rm
    ) ;; rm
  (func $work (;5;) (type 0) ;; rm
    i32.const 3 ;; rm
    drop ;; rm
    ) ;; rm
  (memory (;0;) 1)
  (export "memory" (memory 0))
)
