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
