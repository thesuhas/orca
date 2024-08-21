use log::error;
use orca::ir::function::FunctionBuilder;
use orca::ir::id::{ExportsID, FunctionID};
use orca::ir::module::module_functions::FuncKind::{Import, Local};
use orca::ir::module::module_functions::{ImportedFunction, LocalFunction};
use orca::ir::types::Body;
use orca::{Module, Opcode};
mod common;
use crate::common::check_instrumentation_encoding;

#[test]
fn test_fn_types() {
    let file = "tests/test_inputs/handwritten/modules/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let module = Module::parse(&buff, false).expect("Unable to parse module");

    assert_eq!(
        *module.functions.get_kind(0),
        Import(ImportedFunction::new(0, 2, 0))
    );
    assert_eq!(
        *module.functions.get_kind(1),
        Local(LocalFunction::new(5, 0, Body::default(), vec![]))
    );
    assert_eq!(
        *module.functions.get_kind(2),
        Local(LocalFunction::new(0, 0, Body::default(), vec![]))
    );
}

#[test]
fn test_exports() {
    let file = "tests/test_inputs/instr_testing/modules/function_modification/export_deletion.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    // Get func ID by name
    assert_eq!(
        module.exports.get_func_by_name("add".to_string()).unwrap(),
        1 as FunctionID
    );

    // Get Export ID by name
    assert_eq!(
        module
            .exports
            .get_export_id_by_name("add".to_string())
            .unwrap(),
        0 as ExportsID
    );

    // Check deletion
    let id = module
        .exports
        .get_export_id_by_name("add".to_string())
        .unwrap();
    module.exports.delete(id);

    let result = module.encode();
    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_import_delete() {
    let file = "tests/test_inputs/instr_testing/modules/function_modification/import_delete.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    let id = module.imports.find("bogus".to_string(), "hi".to_string());

    assert!(!id.is_none());

    module.delete_import_func(id.unwrap());

    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_local_fn_delete() {
    let file = "tests/test_inputs/instr_testing/modules/function_modification/local_fn_delete.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    module.functions.delete(2);

    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
#[should_panic]
fn test_panic_call_delete() {
    let file = "tests/test_inputs/handwritten/modules/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    module.functions.delete(1);

    // Should panic here as func 2 calls func 1 which has been deleted
    module.encode();
}

#[test]
fn test_renumber_fn_id() {
    let file =
        "tests/test_inputs/instr_testing/modules/function_modification/local_fn_renumber.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    module.functions.delete(1);

    // Should reencode and get original add.wat file
    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_middle_import_to_local() {
    let file =
        "tests/test_inputs/instr_testing/modules/function_modification/middle_import_to_local.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    let mut builder = FunctionBuilder::new(&*vec![], &*vec![]);
    builder.i32_const(1);
    builder.drop();
    builder.end();

    module.convert_import_fn_to_local(1, builder.local_func(vec![], 1, 0));

    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_first_import_to_local() {
    let file =
        "tests/test_inputs/instr_testing/modules/function_modification/first_import_to_local.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    let mut builder = FunctionBuilder::new(&*vec![], &*vec![]);
    builder.i32_const(1);
    builder.drop();
    builder.end();

    module.convert_import_fn_to_local(0, builder.local_func(vec![], 0, 0));

    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_last_import_to_local() {
    let file =
        "tests/test_inputs/instr_testing/modules/function_modification/last_import_to_local.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    let mut builder = FunctionBuilder::new(&*vec![], &*vec![]);
    builder.i32_const(1);
    builder.drop();
    builder.end();

    module.convert_import_fn_to_local(2, builder.local_func(vec![], 2, 0));

    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_all_import_to_local() {
    let file =
        "tests/test_inputs/instr_testing/modules/function_modification/all_import_to_local.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    // Convert all to local
    let mut first_builder = FunctionBuilder::new(&*vec![], &*vec![]);
    first_builder.i32_const(1);
    first_builder.drop();
    first_builder.end();
    module.convert_import_fn_to_local(0, first_builder.local_func(vec![], 0, 0));

    let mut second_builder = FunctionBuilder::new(&*vec![], &*vec![]);
    second_builder.i32_const(2);
    second_builder.drop();
    second_builder.end();
    module.convert_import_fn_to_local(1, second_builder.local_func(vec![], 1, 0));

    let mut third_builder = FunctionBuilder::new(&*vec![], &*vec![]);
    third_builder.i32_const(3);
    third_builder.drop();
    third_builder.end();
    module.convert_import_fn_to_local(2, third_builder.local_func(vec![], 2, 0));

    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_some_import_to_local() {
    let file =
        "tests/test_inputs/instr_testing/modules/function_modification/some_import_to_local.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    // Convert all to local
    let mut first_builder = FunctionBuilder::new(&*vec![], &*vec![]);
    first_builder.i32_const(1);
    first_builder.drop();
    first_builder.end();
    module.convert_import_fn_to_local(0, first_builder.local_func(vec![], 0, 0));

    let mut second_builder = FunctionBuilder::new(&*vec![], &*vec![]);
    second_builder.i32_const(2);
    second_builder.drop();
    second_builder.end();
    module.convert_import_fn_to_local(1, second_builder.local_func(vec![], 1, 0));

    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_middle_import_to_local_import_delete() {
    let file =
        "tests/test_inputs/instr_testing/modules/function_modification/middle_import_to_local_import_delete.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    let mut builder = FunctionBuilder::new(&*vec![], &*vec![]);
    builder.i32_const(1);
    builder.drop();
    builder.end();

    module.convert_import_fn_to_local(1, builder.local_func(vec![], 1, 0));

    module.delete_import_func(2);
    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_middle_import_to_local_local_delete() {
    let file =
        "tests/test_inputs/instr_testing/modules/function_modification/middle_import_to_local_local_delete.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    let mut builder = FunctionBuilder::new(&*vec![], &*vec![]);
    builder.i32_const(1);
    builder.drop();
    builder.end();

    module.convert_import_fn_to_local(1, builder.local_func(vec![], 1, 0));

    module.delete_import_func(2);
    module.functions.delete(3);

    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_add_import() {
    let file = "tests/test_inputs/instr_testing/modules/function_modification/add_import.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    module.add_import_func("orca".to_string(), "better".to_string(), 2);
    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_middle_local_to_import() {
    let file =
        "tests/test_inputs/instr_testing/modules/function_modification/middle_local_to_import.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    module.convert_local_fn_to_import(2, "orca".to_string(), "better".to_string(), 2);
    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");

    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_first_local_to_import() {
    let file =
        "tests/test_inputs/instr_testing/modules/function_modification/first_local_to_import.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    module.convert_local_fn_to_import(1, "orca".to_string(), "better".to_string(), 2);
    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");

    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_last_local_to_import() {
    let file =
        "tests/test_inputs/instr_testing/modules/function_modification/last_local_to_import.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    module.convert_local_fn_to_import(3, "orca".to_string(), "better".to_string(), 2);
    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");

    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_all_local_to_import() {
    let file =
        "tests/test_inputs/instr_testing/modules/function_modification/all_local_to_import.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    module.convert_local_fn_to_import(3, "all".to_string(), "local".to_string(), 2);
    module.convert_local_fn_to_import(4, "to".to_string(), "import".to_string(), 2);
    module.convert_local_fn_to_import(5, "please".to_string(), "work".to_string(), 2);
    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");

    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_some_local_to_import() {
    let file =
        "tests/test_inputs/instr_testing/modules/function_modification/some_local_to_import.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    module.convert_local_fn_to_import(3, "all".to_string(), "local".to_string(), 2);
    module.convert_local_fn_to_import(4, "to".to_string(), "import".to_string(), 2);
    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");

    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_all_local_to_import_all_import_to_local() {
    let file = "tests/test_inputs/instr_testing/modules/function_modification/all_local_to_import_all_import_to_local.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    // Convert all to local
    let mut first_builder = FunctionBuilder::new(&*vec![], &*vec![]);
    first_builder.i32_const(4);
    first_builder.drop();
    first_builder.end();
    module.convert_import_fn_to_local(0, first_builder.local_func(vec![], 0, 0));

    let mut second_builder = FunctionBuilder::new(&*vec![], &*vec![]);
    second_builder.i32_const(5);
    second_builder.drop();
    second_builder.end();
    module.convert_import_fn_to_local(1, second_builder.local_func(vec![], 1, 0));

    let mut third_builder = FunctionBuilder::new(&*vec![], &*vec![]);
    third_builder.i32_const(6);
    third_builder.drop();
    third_builder.end();
    module.convert_import_fn_to_local(2, third_builder.local_func(vec![], 2, 0));

    module.convert_local_fn_to_import(3, "all".to_string(), "local".to_string(), 2);
    module.convert_local_fn_to_import(4, "to".to_string(), "import".to_string(), 2);
    module.convert_local_fn_to_import(5, "please".to_string(), "work".to_string(), 2);
    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");

    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}
