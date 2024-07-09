use orca::ir::component::Component;
use orca::ir::types::InstrumentType;
use wasmparser::Operator;

pub fn is_same_call(op: &Operator, target: &Operator) -> bool {
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
    component.visitor();
}

// note that the instrumented code does not type check
#[test]
fn test_inject_before() {
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");

    let interested = vec![(
        Operator::Call { function_index: 1 },
        InstrumentType::InstrumentBefore,
    )];

    component.mark_as_instrument(interested.clone());

    component.visitor();

    let injections = vec![(
        Operator::Call { function_index: 1 }, // Original code
        Operator::Call { function_index: 0 }, // Code to inject when original code is encountered
    )];

    component.add_instrumentation(injections.clone());

    for module in component.modules.iter() {
        for body in module.code_sections.iter() {
            for (idx, (instr, ..)) in body.instructions.iter().enumerate() {
                // Now for each instruction, see if it matches to the injected stuff
                for (orig, inject) in injections.iter() {
                    if is_same_call(instr, orig) {
                        // That means, previous instruction must be injected
                        assert_eq!(is_same_call(inject, &body.instructions[idx - 1].0), true);
                    }
                }
            }
        }
    }
    component.visitor();
    let data = component.encode().expect("Unable to encode");
    let out = wasmprinter::print_bytes(data).expect("couldn't translated Wasm to wat");
    // write to file for debugging
    // TODO: the alias should come afte the core instance
    std::fs::write("output.wat", out.clone()).expect("Unable to write to file");
    println!("{}", out);
}

#[test]
fn test_inject_after() {
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");

    let interested = vec![(
        Operator::Call { function_index: 1 },
        InstrumentType::InstrumentAfter,
    )];

    component.mark_as_instrument(interested.clone());

    component.visitor();

    let injections = vec![(
        Operator::Call { function_index: 1 }, // Original code
        Operator::Call { function_index: 0 }, // Code to inject when original code is encountered
    )];

    component.add_instrumentation(injections.clone());

    for module in component.modules.iter() {
        for body in module.code_sections.iter() {
            for (idx, (instr, ..)) in body.instructions.iter().enumerate() {
                // Now for each instruction, see if it matches to the injected stuff
                for (orig, inject) in injections.iter() {
                    if is_same_call(instr, orig) {
                        // That means, previous instruction must be injected
                        assert_eq!(is_same_call(inject, &body.instructions[idx + 1].0), true);
                    }
                }
            }
        }
    }
    component.visitor();
}

#[test]
fn test_inject_alt() {
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");

    let interested = vec![(
        Operator::Call { function_index: 1 },
        InstrumentType::InstrumentAlternate,
    )];

    component.mark_as_instrument(interested.clone());

    component.visitor();

    let injections = vec![(
        Operator::Call { function_index: 1 }, // Original code
        Operator::Call { function_index: 0 }, // Code to inject when original code is encountered
    )];

    component.add_instrumentation(injections.clone());

    // for module in component.modules.iter() {
    //     for body in module.code_sections.iter() {
    //         for (idx, (instr, ..)) in body.instructions.iter().enumerate() {
    //             // Now for each instruction, see if it matches to the injected stuff
    //             for (orig, inject) in injections.iter() {
    //                 if is_same_call(instr, orig) {
    //                     println!("{:?}", body.instructions[idx]);
    //                     // That means, previous instruction must be injected
    //                     assert_eq!(
    //                         is_same_call(inject, &body.instructions[idx].0),
    //                         true
    //                     );
    //                 }
    //             }
    //         }
    //     }
    // }

    // TODO: instrument alternate seems to have the wrong output?
    // `call 1`` should be replaced by `call 0`
    // it has the behavior of `InstrumentBefore` now
    component.visitor();
}
