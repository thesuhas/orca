use orca::ir::id::{ExportsID, FunctionID};
use orca::ir::module::module_functions::FuncKind::{Import, Local};
use orca::ir::module::module_functions::{ImportedFunction, LocalFunction};
use orca::ir::types::Body;
use orca::Module;

#[test]
fn test_fn_types() {
    let file = "tests/test_inputs/handwritten/modules/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let module = Module::parse(&buff, false).expect("Unable to parse module");

    assert_eq!(
        *module.functions.get_kind(0),
        Import(ImportedFunction::new(0, 2))
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
    let file = "tests/test_inputs/handwritten/modules/add.wat";

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

    let m = Module::parse(&result, false).expect("unable to parse");

    assert!(m.exports.get_export_id_by_name("add".to_string()).is_none());
}

#[test]
fn test_import_delete() {
    let file = "tests/test_inputs/handwritten/modules/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    let id = module.imports.find("bogus".to_string(), "hi".to_string());

    assert!(!id.is_none());

    module.delete_import_func(id.unwrap());

    let result = module.encode();

    let m = Module::parse(&result, false).expect("unable to parse");

    assert_eq!(m.functions.len(), 2);
    assert_eq!(m.imports.len(), 0);
}

#[test]
fn test_local_fn_delete() {
    let file = "tests/test_inputs/handwritten/modules/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    module.functions.delete(2);

    let result = module.encode();

    let m = Module::parse(&result, false).expect("unable to parse");

    assert_eq!(m.functions.len(), 2);
    assert_eq!(m.imports.len(), 1);
    assert_eq!(m.functions.get_name(1).unwrap(), "add".to_string());
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
    let file = "tests/test_inputs/handwritten/modules/add_extra.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse module");

    module.functions.delete(1);

    // Should reencode and get original add.wat file
    let result = module.encode();

    let out = wasmprinter::print_bytes(result).expect("error");

    let old_file = "tests/test_inputs/handwritten/modules/add.wat";

    let old_buff = wat::parse_file(old_file).expect("couldn't convert the input wat to Wasm");

    let old_out = wasmprinter::print_bytes(old_buff).expect("uh oh");

    assert_eq!(out, old_out);
}
