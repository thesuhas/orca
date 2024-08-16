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
    let file = "tests/handwritten/modules/add.wat";

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
