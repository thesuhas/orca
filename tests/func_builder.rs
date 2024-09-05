use orca::ir::id::FunctionID;
use orca::opcode::Instrumenter;
use orca::Opcode;
use orca::{Location, Module};
use std::process::Command;

// #[test]
// build factorial from scratch
#[allow(dead_code)]
fn run_fac_orca() {
    // run cargo run in fac_orca dir
    let a = Command::new("cargo")
        .arg("run")
        .current_dir("fac_orca")
        .output()
        .expect("failed to execute process");
    assert!(a.status.success());

    let fac_generated = wasmprinter::print_file("fac_orca/target/out.wasm").unwrap();
    let fac_standard = wasmprinter::print_file("fac_orca/fact.wasm").unwrap();
    assert_eq!(fac_generated, fac_standard);
}

// #[test]
// test start function instrumentation with FunctionModifier
#[allow(dead_code)]
fn run_start_orca() {
    let file_name = "tests/test_inputs/handwritten/modules/start.wat";
    let wasm = wat::parse_file(file_name).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&wasm, false).expect("Unable to parse");

    let start_fun_id = module.start.unwrap();
    let mut function_builder = module.functions.get_fn_modifier(start_fun_id).unwrap();

    function_builder
        .before_at(Location::Module {
            func_idx: FunctionID(0), // not used
            instr_idx: 0,
        })
        .i32_const(1);

    let result = module.encode();
    let out = wasmprinter::print_bytes(result.clone()).expect("couldn't translate Wasm to wat");
    println!("{}", out);
}

#[ignore]
#[test]
// test start function instrumentation with FunctionModifier
fn run_start_orca_default() {
    let file_name = "tests/test_inputs/handwritten/modules/start.wat";
    let wasm = wat::parse_file(file_name).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&wasm, false).expect("Unable to parse");

    let start_fun_id = module.start.unwrap();
    let mut function_builder = module.functions.get_fn_modifier(start_fun_id).unwrap();

    function_builder.i32_const(1);

    let result = module.encode();
    let out = wasmprinter::print_bytes(result.clone()).expect("couldn't translate Wasm to wat");
    println!("{}", out);
}
