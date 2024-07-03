use orca::ir::{Component, InstrumentType};
use wasmparser::Operator;

#[test]
fn test_mark_as_instrumented() {
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let interested = vec![(Operator::Call {function_index: 1}, InstrumentType::InstrumentBefore)];
    component.mark_as_instrument(interested);
    println!("aha");
}