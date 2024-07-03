use orca::ir::{Component, InstrumentType};
use wasmparser::Operator;

#[test]
fn test_mark_as_instrumented() {
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let interested = vec![(Operator::Call {function_index: 1}, InstrumentType::InstrumentBefore)];
    component.mark_as_instrument(interested.clone());

    for (_index, module) in component.modules.iter().enumerate() {
        for (_idx, body) in module.code_sections.iter().enumerate() {
            for (_idx, (instr, instrumented)) in body.instructions.iter().enumerate() {
                for (op, instr_ty) in interested.iter() {
                    if std::mem::discriminant(op) == std::mem::discriminant(instr) {
                        assert_eq!(instr_ty, instrumented);
                    }
                }
            }
        }
    }
}