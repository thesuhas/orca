(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (result i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (param i32 f64 i32) (result i32 f64 i32)))
  (type (;5;) (func (param i32) (result i32)))
  (type (;6;) (func (param i32) (result i32 i32)))
  (type (;7;) (func (result i32 i64 i32)))
  (type (;8;) (func (result i32 i32)))
  (type (;9;) (func (result f32 f32)))
  (type (;10;) (func (param i32) (result i32 i32 i64)))
  (type (;11;) (func (result i32 i32 i64)))
  (type (;12;) (func (param i32 i32) (result i32 i32)))
  (type (;13;) (func (param i64 i64 i32) (result i64 i32)))
  (type (;14;) (func (param i64 i64) (result i64)))
  (type (;15;) (func (param i64) (result i64)))
  ;; imported function
  (import "env" "id" (func (param i32) (result i32)))
  ;; todo -- test with imported memory
  (import "env" "memory" (memory 1))
  ;; imported global
  (import "test" "global-i32" (global f64))
  ;; imported table
  (import "spectest" "table" (table $tab 10 20 funcref))
  (func (;1;) (type 1))
  (func (;2;) (type 3) (param i32)
    local.get 0
    if  ;; label = @1
    end
    local.get 0
    if  ;; label = @1
    end
    local.get 0
    if  ;; label = @1
    end
    local.get 0
    if  ;; label = @1
    end)
  (func (;3;) (type 5) (param i32) (result i32)
    local.get 0
    if  ;; label = @1
      nop
    end
    local.get 0
    if  ;; label = @1
      nop
    else
      nop
    end
    local.get 0
    if (result i32)  ;; label = @1
      i32.const 7
    else
      i32.const 8
    end)
  (func (;4;) (type 6) (param i32) (result i32 i32)
    local.get 0
    if  ;; label = @1
      call 1
      call 1
      call 1
    end
    local.get 0
    if  ;; label = @1
    else
      call 1
      call 1
      call 1
    end
    local.get 0
    if (result i32)  ;; label = @1
      call 1
      call 1
      i32.const 8
      call 1
    else
      call 1
      call 1
      i32.const 9
      call 1
    end
    local.get 0
    if (result i32 i64 i32)  ;; label = @1
      call 1
      call 1
      i32.const 1
      call 1
      call 1
      call 1
      i64.const 2
      call 1
      call 1
      call 1
      i32.const 3
      call 1
    else
      call 1
      call 1
      i32.const -1
      call 1
      call 1
      call 1
      i64.const -2
      call 1
      call 1
      call 1
      i32.const -3
      call 1
    end
    drop
    drop)
  (func (;5;) (type 0) (param i32 i32) (result i32)
    local.get 0
    if (result i32)  ;; label = @1
      local.get 1
      if  ;; label = @2
        call 1
        block  ;; label = @3
        end
        nop
      end
      local.get 1
      if  ;; label = @2
      else
        call 1
        block  ;; label = @3
        end
        nop
      end
      local.get 1
      if (result i32)  ;; label = @2
        call 1
        i32.const 9
      else
        call 1
        i32.const 10
      end
    else
      local.get 1
      if  ;; label = @2
        call 1
        block  ;; label = @3
        end
        nop
      end
      local.get 1
      if  ;; label = @2
      else
        call 1
        block  ;; label = @3
        end
        nop
      end
      local.get 1
      if (result i32)  ;; label = @2
        call 1
        i32.const 10
      else
        call 1
        i32.const 11
      end
    end)
  (func (;6;) (type 5) (param i32) (result i32)
    local.get 0
    if (result i32)  ;; label = @1
      call 1
      i32.const 1
    else
      call 1
      i32.const 0
    end
    i32.const 2
    i32.const 3
    select)
  (func (;7;) (type 5) (param i32) (result i32)
    i32.const 2
    local.get 0
    if (result i32)  ;; label = @1
      call 1
      i32.const 1
    else
      call 1
      i32.const 0
    end
    i32.const 3
    select)
  (func (;8;) (type 5) (param i32) (result i32)
    i32.const 2
    i32.const 3
    local.get 0
    if (result i32)  ;; label = @1
      call 1
      i32.const 1
    else
      call 1
      i32.const 0
    end
    select)
  (func (;9;) (type 5) (param i32) (result i32)
    loop (result i32)  ;; label = @1
      local.get 0
      if (result i32)  ;; label = @2
        call 1
        i32.const 1
      else
        call 1
        i32.const 0
      end
      call 1
      call 1
    end)
  (func (;10;) (type 5) (param i32) (result i32)
    loop (result i32)  ;; label = @1
      call 1
      local.get 0
      if (result i32)  ;; label = @2
        call 1
        i32.const 1
      else
        call 1
        i32.const 0
      end
      call 1
    end)
  (func (;11;) (type 5) (param i32) (result i32)
    loop (result i32)  ;; label = @1
      call 1
      call 1
      local.get 0
      if (result i32)  ;; label = @2
        call 1
        i32.const 1
      else
        call 1
        i32.const 0
      end
    end)
  (func (;12;) (type 5) (param i32) (result i32)
    local.get 0
    if (result i32)  ;; label = @1
      i32.const 1
    else
      i32.const 0
    end
    if (result i32)  ;; label = @1
      call 1
      i32.const 2
    else
      call 1
      i32.const 3
    end)
  (func (;13;) (type 5) (param i32) (result i32)
    block (result i32)  ;; label = @1
      local.get 0
      if (result i32)  ;; label = @2
        call 1
        i32.const 1
      else
        call 1
        i32.const 0
      end
      i32.const 2
      br_if 0 (;@1;)
      i32.const 3
      return
    end)
  (func (;14;) (type 5) (param i32) (result i32)
    block (result i32)  ;; label = @1
      i32.const 2
      local.get 0
      if (result i32)  ;; label = @2
        call 1
        i32.const 1
      else
        call 1
        i32.const 0
      end
      br_if 0 (;@1;)
      i32.const 3
      return
    end)
  (func (;15;) (type 5) (param i32) (result i32)
    block (result i32)  ;; label = @1
      local.get 0
      if (result i32)  ;; label = @2
        call 1
        i32.const 1
      else
        call 1
        i32.const 0
      end
      i32.const 2
      br_table 0 (;@1;) 0 (;@1;)
    end)
  (func (;16;) (type 5) (param i32) (result i32)
    block (result i32)  ;; label = @1
      i32.const 2
      local.get 0
      if (result i32)  ;; label = @2
        call 1
        i32.const 1
      else
        call 1
        i32.const 0
      end
      br_table 0 (;@1;) 0 (;@1;)
    end)
  (func (;17;) (type 0) (param i32 i32) (result i32)
    local.get 0)
  (func (;18;) (type 5) (param i32) (result i32)
    block (result i32)  ;; label = @1
      local.get 0
      if (result i32)  ;; label = @2
        call 1
        i32.const 1
      else
        call 1
        i32.const 0
      end
      i32.const 2
      i32.const 0
      call_indirect (type 0)
    end)
  (func (;19;) (type 5) (param i32) (result i32)
    block (result i32)  ;; label = @1
      i32.const 2
      local.get 0
      if (result i32)  ;; label = @2
        call 1
        i32.const 1
      else
        call 1
        i32.const 0
      end
      i32.const 0
      call_indirect (type 0)
    end)
  (func (;20;) (type 5) (param i32) (result i32)
    block (result i32)  ;; label = @1
      i32.const 2
      i32.const 0
      local.get 0
      if (result i32)  ;; label = @2
        call 1
        i32.const 1
      else
        call 1
        i32.const 0
      end
      call_indirect (type 0)
    end)
  (func (;21;) (type 3) (param i32)
    local.get 0
    if (result i32)  ;; label = @1
      call 1
      i32.const 1
    else
      call 1
      i32.const 0
    end
    i32.const 2
    i32.store)
  (func (;22;) (type 3) (param i32)
    i32.const 2
    local.get 0
    if (result i32)  ;; label = @1
      call 1
      i32.const 1
    else
      call 1
      i32.const 0
    end
    i32.store)
  (func (;23;) (type 5) (param i32) (result i32)
    local.get 0
    if (result i32)  ;; label = @1
      i32.const 1
    else
      i32.const 0
    end
    memory.grow)
  (func (;24;) (type 5) (param i32) (result i32)
    local.get 0)
  (func (;25;) (type 5) (param i32) (result i32)
    local.get 0
    if (result i32)  ;; label = @1
      i32.const 1
    else
      i32.const 0
    end
    call 24)
  (func (;26;) (type 5) (param i32) (result i32)
    local.get 0
    if (result i32)  ;; label = @1
      i32.const 1
    else
      i32.const 0
    end
    return)
  (func (;27;) (type 3) (param i32)
    local.get 0
    if (result i32)  ;; label = @1
      i32.const 1
    else
      i32.const 0
    end
    drop)
  (func (;28;) (type 5) (param i32) (result i32)
    block (result i32)  ;; label = @1
      local.get 0
      if (result i32)  ;; label = @2
        i32.const 1
      else
        i32.const 0
      end
      br 0 (;@1;)
    end)
  (func (;29;) (type 5) (param i32) (result i32)
    (local i32)
    local.get 0
    if (result i32)  ;; label = @1
      i32.const 1
    else
      i32.const 0
    end
    local.set 0
    local.get 0)
  (func (;30;) (type 5) (param i32) (result i32)
    local.get 0
    if (result i32)  ;; label = @1
      i32.const 1
    else
      i32.const 0
    end
    local.tee 0)
  (func (;31;) (type 5) (param i32) (result i32)
    local.get 0
    if (result i32)  ;; label = @1
      i32.const 1
    else
      i32.const 0
    end
    global.set 1
    global.get 1)
  (func (;32;) (type 5) (param i32) (result i32)
    local.get 0
    if (result i32)  ;; label = @1
      i32.const 11
    else
      i32.const 10
    end
    i32.load)
  (func (;33;) (type 5) (param i32) (result i32)
    local.get 0
    if (result i32)  ;; label = @1
      call 1
      i32.const 13
    else
      call 1
      i32.const -13
    end
    i32.ctz)
  (func (;34;) (type 0) (param i32 i32) (result i32)
    local.get 0
    if (result i32)  ;; label = @1
      call 1
      i32.const 3
    else
      call 1
      i32.const -3
    end
    local.get 1
    if (result i32)  ;; label = @1
      call 1
      i32.const 4
    else
      call 1
      i32.const -5
    end
    i32.mul)
  (func (;35;) (type 5) (param i32) (result i32)
    local.get 0
    if (result i32)  ;; label = @1
      call 1
      i32.const 13
    else
      call 1
      i32.const 0
    end
    i32.eqz)
  (func (;36;) (type 0) (param i32 i32) (result i32)
    local.get 0
    if (result f32)  ;; label = @1
      call 1
      f32.const 0x1.8p+1 (;=3;)
    else
      call 1
      f32.const -0x1.8p+1 (;=-3;)
    end
    local.get 1
    if (result f32)  ;; label = @1
      call 1
      f32.const 0x1p+2 (;=4;)
    else
      call 1
      f32.const -0x1p+2 (;=-4;)
    end
    f32.gt)
  (func (;37;) (type 5) (param i32) (result i32)
    local.get 0
    if (result i32 i32)  ;; label = @1
      call 1
      i32.const 3
      call 1
      i32.const 4
    else
      call 1
      i32.const 3
      call 1
      i32.const -4
    end
    i32.mul)
  (func (;38;) (type 5) (param i32) (result i32)
    local.get 0
    if (result f32 f32)  ;; label = @1
      call 1
      f32.const 0x1.8p+1 (;=3;)
      call 1
      f32.const 0x1.8p+1 (;=3;)
    else
      call 1
      f32.const -0x1p+1 (;=-2;)
      call 1
      f32.const -0x1.8p+1 (;=-3;)
    end
    f32.gt)
  (func (;39;) (type 5) (param i32) (result i32)
    local.get 0
    if (result i32 i32)  ;; label = @1
      call 1
      i32.const 3
      call 1
      i32.const 4
    else
      call 1
      i32.const -3
      call 1
      i32.const -4
    end
    i32.const 5
    i32.add
    i32.mul)
  (func (;40;) (type 2) (result i32)
    i32.const 1
    if  ;; label = @1
      br 0 (;@1;)
      unreachable
    end
    i32.const 1
    if  ;; label = @1
      br 0 (;@1;)
      unreachable
    else
      unreachable
    end
    i32.const 0
    if  ;; label = @1
      unreachable
    else
      br 0 (;@1;)
      unreachable
    end
    i32.const 1
    if  ;; label = @1
      i32.const 1
      br_if 0 (;@1;)
      unreachable
    end
    i32.const 1
    if  ;; label = @1
      i32.const 1
      br_if 0 (;@1;)
      unreachable
    else
      unreachable
    end
    i32.const 0
    if  ;; label = @1
      unreachable
    else
      i32.const 1
      br_if 0 (;@1;)
      unreachable
    end
    i32.const 1
    if  ;; label = @1
      i32.const 0
      br_table 0 (;@1;)
      unreachable
    end
    i32.const 1
    if  ;; label = @1
      i32.const 0
      br_table 0 (;@1;)
      unreachable
    else
      unreachable
    end
    i32.const 0
    if  ;; label = @1
      unreachable
    else
      i32.const 0
      br_table 0 (;@1;)
      unreachable
    end
    i32.const 19)
  (func (;41;) (type 5) (param i32) (result i32)
    local.get 0
    if (result i32)  ;; label = @1
      i32.const 18
      br 0 (;@1;)
      i32.const 19
    else
      i32.const 21
      br 0 (;@1;)
      i32.const 20
    end)
  (func (;42;) (type 10) (param i32) (result i32 i32 i64)
    local.get 0
    if (result i32 i32 i64)  ;; label = @1
      i32.const 18
      i32.const -18
      i64.const 18
      br 0 (;@1;)
      i32.const 19
      i32.const -19
      i64.const 19
    else
      i32.const -18
      i32.const 18
      i64.const -18
      br 0 (;@1;)
      i32.const -19
      i32.const 19
      i64.const -19
    end)
  (func (;43;) (type 5) (param i32) (result i32)
    i32.const 1
    local.get 0
    if (param i32) (result i32)  ;; label = @1
      i32.const 2
      i32.add
    else
      i32.const -2
      i32.add
    end)
  (func (;44;) (type 5) (param i32) (result i32)
    i32.const 1
    i32.const 2
    local.get 0
    if (param i32 i32) (result i32)  ;; label = @1
      i32.add
    else
      i32.sub
    end)
  (func (;45;) (type 5) (param i32) (result i32)
    i32.const 1
    i32.const 2
    local.get 0
    if (param i32 i32) (result i32 i32)  ;; label = @1
    end
    i32.add)
  (func (;46;) (type 5) (param i32) (result i32)
    i32.const 1
    local.get 0
    if (param i32) (result i32)  ;; label = @1
      i32.const 2
      i32.add
      br 0 (;@1;)
    else
      i32.const -2
      i32.add
      br 0 (;@1;)
    end)
  (func (;47;) (type 5) (param i32) (result i32)
    i32.const 1
    i32.const 2
    local.get 0
    if (param i32 i32) (result i32)  ;; label = @1
      i32.add
      br 0 (;@1;)
    else
      i32.sub
      br 0 (;@1;)
    end)
  (func (;48;) (type 5) (param i32) (result i32)
    i32.const 1
    i32.const 2
    local.get 0
    if (param i32 i32) (result i32 i32)  ;; label = @1
      br 0 (;@1;)
    end
    i32.add)
  (func (;49;) (type 5) (param i32) (result i32)
    (local i32)
    block (result i32)  ;; label = @1
      i32.const 1
      local.set 1
      local.get 0
    end
    if  ;; label = @1
      local.get 1
      i32.const 3
      i32.mul
      local.set 1
      local.get 1
      i32.const 5
      i32.sub
      local.set 1
      local.get 1
      i32.const 7
      i32.mul
      local.set 1
      br 0 (;@1;)
      local.get 1
      i32.const 100
      i32.mul
      local.set 1
    else
      local.get 1
      i32.const 5
      i32.mul
      local.set 1
      local.get 1
      i32.const 7
      i32.sub
      local.set 1
      local.get 1
      i32.const 3
      i32.mul
      local.set 1
      br 0 (;@1;)
      local.get 1
      i32.const 1000
      i32.mul
      local.set 1
    end
    local.get 1)
  (func (;50;) (type 13) (param i64 i64 i32) (result i64 i32)
    (local i64)
    local.get 0
    local.get 1
    i64.add
    local.get 2
    i64.extend_i32_u
    i64.add
    local.set 3
    local.get 3
    local.get 3
    local.get 0
    i64.lt_u
    return)
  (func (;51;) (type 14) (param i64 i64) (result i64)
    local.get 0
    local.get 1
    i32.const 0
    call 50
    if (param i64) (result i64)  ;; label = @1
      drop
      i64.const -1
    end)
  (func (;52;) (type 1)
    i32.const 1
    if  ;; label = @1
    end
    i32.const 1
    if (result i32)  ;; label = @1
      i32.const 0
    else
      i32.const 2
    end
    i32.const 1
    if (param i32)  ;; label = @1
      drop
    else
      drop
    end
    i32.const 0
    f64.const 0x0p+0 (;=0;)
    i32.const 0
    i32.const 1
    if (param i32 f64 i32) (result i32 f64 i32)  ;; label = @1
    end
    drop
    drop
    drop
    i32.const 1
    if (result i32)  ;; label = @1
      i32.const 0
    else
      i32.const 2
    end
    i32.const 1
    if (param i32)  ;; label = @1
      drop
    else
      drop
    end
    i32.const 0
    f64.const 0x0p+0 (;=0;)
    i32.const 0
    i32.const 1
    if (param i32 f64 i32) (result i32 f64 i32)  ;; label = @1
    end
    drop
    drop
    drop)
  (func (;53;) (type 1)
    i32.const 0
    if  ;; label = @1
    end
    i32.const 1
    i32.eqz
    if  ;; label = @1
    end)
  (table (;0;) 1 1 funcref)
  (memory (;1;) 1)
  (global (;1;) (mut i32) (i32.const 10))
  (global (;2;) (mut i32) (i32.const 10)) ;; unused
  (export "empty" (func 1))
  (export "singular" (func 2))
  (export "multi" (func 3))
  (export "nested" (func 4))
  (export "as-select-first" (func 5))
  (export "as-select-mid" (func 6))
  (export "as-select-last" (func 7))
  (export "as-loop-first" (func 8))
  (export "as-loop-mid" (func 9))
  (export "as-loop-last" (func 10))
  (export "as-if-condition" (func 11))
  (export "as-br_if-first" (func 12))
  (export "as-br_if-last" (func 13))
  (export "as-br_table-first" (func 14))
  (export "as-br_table-last" (func 15))
  (export "as-call_indirect-first" (func 17))
  (export "as-call_indirect-mid" (func 18))
  (export "as-call_indirect-last" (func 19))
  (export "as-store-first" (func 20))
  (export "as-store-last" (func 21))
  (export "as-memory.grow-value" (func 22))
  (export "as-call-value" (func 24))
  (export "as-return-value" (func 25))
  (export "as-drop-operand" (func 26))
  (export "as-br-value" (func 27))
  (export "as-local.set-value" (func 28))
  (export "as-local.tee-value" (func 29))
  (export "as-global.set-value" (func 30))
  (export "as-load-operand" (func 31))
  (export "as-unary-operand" (func 32))
  (export "as-binary-operand" (func 33))
  (export "as-test-operand" (func 34))
  (export "as-compare-operand" (func 35))
  (export "as-binary-operands" (func 36))
  (export "as-compare-operands" (func 37))
  (export "as-mixed-operands" (func 38))
  (export "break-bare" (func 39))
  (export "break-value" (func 40))
  (export "break-multi-value" (func 41))
  (export "param" (func 42))
  (export "params" (func 43))
  (export "params-id" (func 44))
  (export "param-break" (func 45))
  (export "params-break" (func 46))
  (export "params-id-break" (func 47))
  (export "effects" (func 48))
  (export "add64_u_with_carry" (func 49))
  (export "add64_u_saturated" (func 50))
  (export "type-use" (func 51))
  (export "atypical-condition" (func 52))
  (elem (;0;) (i32.const 0) func 16))
