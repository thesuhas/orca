use orca::ir::{Component, InstrumentType};
use wasmparser::Operator;

pub fn compare_operator(op: &Operator, target: &Operator) -> bool {
    match (op, target) {
        (
            Operator::Call {
                function_index: idx1,
            },
            Operator::Call {
                function_index: idx2,
            },
        ) => idx1 == idx2,
        _ => false,
    }
}

#[test]
fn test_mark_as_instrumented() {
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let interested = vec![(
        Operator::Call { function_index: 1 },
        InstrumentType::InstrumentBefore,
    )];
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

#[test]
fn test_inject_code() {
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");

    let interested = vec![(
        Operator::Call { function_index: 1 },
        InstrumentType::InstrumentBefore,
    )];

    component.mark_as_instrument(interested.clone());

    let injections = vec![(
        Operator::Call { function_index: 1 }, // Original code
        Operator::Call { function_index: 0 }, // Code to inject when original code is encountered
    )];

    component.add_instrumentation(injections.clone());

    for module in component.modules.iter() {
        for body in module.code_sections.iter() {
            for (idx, (instr, ty)) in body.instructions.iter().enumerate() {
                // Now for each instruction, see if it matches to the injected stuff
                for (orig, inject) in injections.iter() {
                    if compare_operator(instr, orig) {
                        // That means, previous instruction must be injected
                        assert_eq!(
                            compare_operator(inject, &body.instructions[idx - 1].0),
                            true
                        );
                    }
                }
            }
        }
    }
}
