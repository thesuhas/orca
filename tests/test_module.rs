use orca::ir::types::FuncKind::{Import, Local};
use orca::Module;

#[test]
fn test_fn_types() {
    let file = "tests/handwritten/modules/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let module = Module::parse(&buff, false).expect("Unable to parse module");

    assert_eq!(module.get_fn_kind(0), Some(Import(2)));
    assert_eq!(module.get_fn_kind(1), Some(Local(5)));
    assert_eq!(module.get_fn_kind(2), Some(Local(0)));
}
