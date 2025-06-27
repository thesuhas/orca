use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::{FunctionID, TypeID};
use orca_wasm::iterator::iterator_trait::Iterator;
use orca_wasm::iterator::module_iterator::ModuleIterator;
use orca_wasm::opcode::Instrumenter;
use orca_wasm::{DataType, Opcode};
use orca_wasm::{Location, Module};
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
#[test]
// test start function instrumentation with FunctionModifier
fn add_import_and_local_fn_then_iterate() {
    let file_name = "tests/test_inputs/handwritten/modules/_start.wat";
    let wasm = wat::parse_file(file_name).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&wasm, false).expect("Unable to parse");
    // add an imported function AND THEN a new local function
    module.add_import_func("new".to_string(), "import".to_string(), TypeID(0));
    assert_eq!(module.num_import_func(), 1);

    let params = vec![];
    let results = vec![DataType::I32];

    let mut new_func = FunctionBuilder::new(&params, &results);
    new_func.i32_const(1);
    new_func.finish_module(&mut module);

    // now iterate over module
    let mut mod_it = ModuleIterator::new(&mut module, &vec![]);
    loop {
        let _op = mod_it.curr_op();
        if mod_it.next().is_none() {
            break;
        };
    }

    let result = module.encode();
    let out = wasmprinter::print_bytes(result.clone()).expect("couldn't translate Wasm to wat");
    println!("{}", out);
}
