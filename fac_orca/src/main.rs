use orca::ir::module::*;
use wasmparser::ValType;

fn main() {

    let mut module = Module::new();
    let log_type_id = module.add_type(&[ValType::I32], &[]);
    let log_func_id = module.add_import_func("env", "log", log_type_id);

    // let mut factorial = FunctionBuilder::new(&mut module.types, &[ValType::I32], &[ValType::I32]);

    // // Create our parameter and our two locals.
    // let n = module.locals.add(ValType::I32);
    // let i = module.locals.add(ValType::I32);
    // let res = module.locals.add(ValType::I32);

    // factorial
    //     // Enter the function's body.
    //     .func_body()
    //     // (local.set $i (local.get $n))
    //     .local_get(n)
    //     .local_set(i)
    //     // (local.set $res (i32.const 1))
    //     .i32_const(1)
    //     .local_set(res)
    //     .block(None, |done| {
    //         let done_id = done.id();
    //         done.loop_(None, |loop_| {
    //             let loop_id = loop_.id();
    //             loop_
    //                 // (call $log (local.get $res))
    //                 .local_get(res)
    //                 .call(log)
    //                 // (i32.eq (local.get $i) (i32.const 0))
    //                 .local_get(i)
    //                 .i32_const(0)
    //                 .binop(BinaryOp::I32Eq)
    //                 .if_else(
    //                     None,
    //                     |then| {
    //                         // (then (br $done))
    //                         then.br(done_id);
    //                     },
    //                     |else_| {
    //                         else_
    //                             // (local.set $res (i32.mul (local.get $i) (local.get $res)))
    //                             .local_get(i)
    //                             .local_get(res)
    //                             .binop(BinaryOp::I32Mul)
    //                             .local_set(res)
    //                             // (local.set $i (i32.sub (local.get $i) (i32.const 1))))
    //                             .local_get(i)
    //                             .i32_const(1)
    //                             .binop(BinaryOp::I32Sub)
    //                             .local_set(i);
    //                     },
    //                 )
    //                 .br(loop_id);
    //         });
    //     })
    //     .local_get(res);

    // let factorial = factorial.finish(vec![n], &mut module.funcs);

    // // Export the `factorial` function.
    // module.exports.add("factorial", factorial);

    // module.emit_wasm("target/out.wasm").unwrap();
    let bytes = module.encode_only_module();
    let wat = wasmprinter::print_bytes(&bytes).unwrap();
    println!("{}", wat);
}
