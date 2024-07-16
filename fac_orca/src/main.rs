//! Contructs a wasm module from scratch with orca
//! from https://github.com/rustwasm/walrus/blob/main/examples/build-wasm-from-scratch.rs
//!
//! You can run this wasm file with `fact.js`

use orca::ir::module::*;
use wasmparser::ValType;
use orca::ir::function::FunctionBuilder;

fn main() {

    let mut module = Module::new();
    let log_type_id = module.add_type(&[ValType::I32], &[]);
    let log_func_id = module.add_import_func("env", "log", log_type_id);

    let mut factorial = FunctionBuilder::new(&[ValType::I32], &[ValType::I32]);

    // Create our parameter and our two locals.
    let n = 0u32;
    let i = factorial.add_local(ValType::I32);
    let res = factorial.add_local(ValType::I32);

    factorial
        // (local.set $i (local.get $n))
        .local_get(n)
        .local_set(i)
        // (local.set $res (i32.const 1))
        .i32_const(1)
        .local_set(res)
            .block(wasmparser::BlockType::Empty) // label 1
                .loop_stmt(wasmparser::BlockType::Empty) // label 2
                    // (call $log (local.get $res))
                    .local_get(res)
                    .call(log_func_id)
                    // (i32.eq (local.get $i) (i32.const 0))
                    .local_get(i)
                    .i32_const(0)
                    .binop(wasmparser::Operator::I32Eq)
                    .if_stmt(wasmparser::BlockType::Empty)
                        // (then (br to outside block @1))
                        .br(2)
                    .else_stmt()
                        // (local.set $res (i32.mul (local.get $i) (local.get $res)))
                        .local_get(i)
                        .local_get(res)
                        .binop(wasmparser::Operator::I32Mul)
                        .local_set(res)
                        // (local.set $i (i32.sub (local.get $i) (i32.const 1))))
                        .local_get(i)
                        .i32_const(1)
                        .binop(wasmparser::Operator::I32Sub)
                        .local_set(i)
                    .end()
                    .br(0) // back to loop @2
                .end()
            .end()
        .local_get(res);

    // let factorial = factorial.finish(vec![n], &mut module.funcs);
    let fact_id = factorial.finish(&mut module);

    // Export the `factorial` function.
    module.add_export_func("factorial", fact_id);

    module.emit_wasm("target/out.wasm").unwrap();

    // debug use: print the wat file
    // let bytes = module.encode_only_module();
    // let wat = wasmprinter::print_bytes(&bytes).unwrap();
    // println!("{}", wat);
}
