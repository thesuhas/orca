use log::{error, trace};
use orca::iterator::component_iterator::ComponentIterator;
use orca::iterator::iterator_trait::{Instrumenter, Iterator};
use orca::iterator::module_iterator::ModuleIterator;
use orca::opcode::Inject;
use orca::{Component, Location, Module, Opcode};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator as StdIter;
use std::mem::discriminant;
use wasmparser::Operator;
mod common;
use crate::common::check_instrumentation_encoding;

#[test]
fn no_injection() {
    let file = "tests/test_inputs/handwritten/components/add.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&mut component, HashMap::new());

    let interested = Operator::Call { function_index: 1 };

    loop {
        let op = comp_it.curr_op();
        let instr_mode = comp_it.curr_instrument_mode();

        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = comp_it.curr_loc()
        {
            trace!(
                "Mod: {}, Func: {}, +{}: {:?}, {:?}",
                mod_idx,
                func_idx,
                instr_idx,
                op,
                instr_mode
            );
            if *comp_it.curr_op().unwrap() == interested {
                comp_it.before();
            }
            if comp_it.next().is_none() {
                break;
            };
        } else {
            panic!("Should've gotten Component Location!");
        }
    }

    comp_it.reset();

    loop {
        let op = comp_it.curr_op();
        let instr_mode = comp_it.curr_instrument_mode();
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = comp_it.curr_loc()
        {
            if *comp_it.curr_op().unwrap() == interested {
                assert_ne!(discriminant(instr_mode), discriminant(&None));
            }

            trace!(
                "Mod: {}, Func: {}, +{}: {:?}, {:?}",
                mod_idx,
                func_idx,
                instr_idx,
                op,
                instr_mode
            );

            if comp_it.next().is_none() {
                break;
            };
        } else {
            panic!("Should've gotten Component Location!");
        }
    }
}

#[test]
fn iterator_inject_i32_before() {
    let file = "tests/test_inputs/instr_testing/components/add-inject_i32_before.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&mut component, HashMap::new());

    let interested = Operator::Call { function_index: 1 };

    loop {
        let op = comp_it.curr_op();
        let instr_mode = comp_it.curr_instrument_mode();

        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = comp_it.curr_loc()
        {
            trace!(
                "Mod: {}, Func: {}, +{}: {:?}, {:?}",
                mod_idx,
                func_idx,
                instr_idx,
                op,
                instr_mode
            );
            if *comp_it.curr_op().unwrap() == interested {
                comp_it.before().i32_const(1);
            }
            if comp_it.next().is_none() {
                break;
            };
        } else {
            panic!("Should've gotten Component Location!");
        }
    }
    comp_it.reset();

    let result = component.encode();
    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn iterator_inject_all_variations() {
    let file = "tests/test_inputs/instr_testing/components/add-inject_all_variations.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&mut component, HashMap::new());

    let after = Operator::Call { function_index: 1 };
    let before = Operator::Drop;
    let alternate = Operator::I32Const { value: 2 };

    loop {
        let op = comp_it.curr_op();
        let instr_mode = comp_it.curr_instrument_mode();

        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = comp_it.curr_loc()
        {
            trace!(
                "Mod: {}, Func: {}, +{}: {:?}, {:?}",
                mod_idx,
                func_idx,
                instr_idx,
                op,
                instr_mode
            );
            if *comp_it.curr_op().unwrap() == before {
                comp_it.before().call(0);
            }

            if *comp_it.curr_op().unwrap() == after {
                comp_it.after().i32_const(0);
            }

            if *comp_it.curr_op().unwrap() == alternate {
                comp_it.alternate().i32_const(3);
            }

            if comp_it.next().is_none() {
                break;
            };
        } else {
            panic!("Should've gotten Component Location!");
        }
    }
    let result = component.encode();
    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

#[test]
fn test_inject_locals() {
    let file = "tests/test_inputs/instr_testing/modules/add-inject_locals.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut is_first = true;
    while is_first || mod_it.next().is_some() {
        let op = mod_it.curr_op();
        if let Location::Module {
            func_idx,
            instr_idx,
        } = mod_it.curr_loc()
        {
            println!("Func: {}, {}: {:?},", func_idx, instr_idx, op);

            if mod_it.curr_op().unwrap() == &Operator::I32Add {
                let local_id = mod_it.add_local(orca::ir::types::DataType::I32);
                println!("new Local ID: {:?}", local_id);
            }

            if mod_it.curr_op().unwrap() == &(Operator::I32Const { value: 2 }) {
                let local_id = mod_it.add_local(orca::ir::types::DataType::I32);
                println!("new Local ID: {:?}", local_id);
            }
        } else {
            panic!("Should've gotten Module Location!");
        }

        is_first = false;
    }

    let result = module.encode();
    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

// ==== SEMANTIC AFTER ====
#[test]
fn test_semantic_after_complex_mult_nested_diff_opcodes() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/complex_mult_nested_diff_opcodes.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut block_body = vec![];
    block_body.push(Operator::I32Const { value: 12 });
    block_body.push(Operator::Drop);

    let mut loop_body = vec![];
    loop_body.push(Operator::I32Const { value: 23 });
    loop_body.push(Operator::Drop);

    let mut if_body = vec![];
    if_body.push(Operator::I32Const { value: 34 });
    if_body.push(Operator::Drop);

    let mut else_body = vec![];
    else_body.push(Operator::I32Const { value: 45 });
    else_body.push(Operator::Drop);

    let mut br_body = vec![];
    br_body.push(Operator::I32Const { value: 56 });
    br_body.push(Operator::Drop);

    let mut br_if_body = vec![];
    br_if_body.push(Operator::I32Const { value: 67 });
    br_if_body.push(Operator::Drop);

    let mut br_table_body = vec![];
    br_table_body.push(Operator::I32Const { value: 78 });
    br_table_body.push(Operator::Drop);

    let ops_of_interest = vec![
        (SupportedOperators::Block, block_body),
        (SupportedOperators::Loop, loop_body),
        (SupportedOperators::If, if_body),
        (SupportedOperators::Else, else_body),
        (SupportedOperators::Br, br_body),
        (SupportedOperators::BrIf, br_if_body),
        (SupportedOperators::BrTable, br_table_body),
    ];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_medium_1br() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/medium_1br.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut block_body = vec![];
    block_body.push(Operator::I32Const { value: 12 });
    block_body.push(Operator::Drop);

    let mut br_body = vec![];
    br_body.push(Operator::I32Const { value: 23 });
    br_body.push(Operator::Drop);

    let ops_of_interest = vec![
        (SupportedOperators::Block, block_body),
        (SupportedOperators::Br, br_body),
    ];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_medium_1br_if() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/medium_1br_if.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut block_body = vec![];
    block_body.push(Operator::I32Const { value: 12 });
    block_body.push(Operator::Drop);

    let mut br_if_body = vec![];
    br_if_body.push(Operator::I32Const { value: 23 });
    br_if_body.push(Operator::Drop);

    let ops_of_interest = vec![
        (SupportedOperators::Block, block_body),
        (SupportedOperators::BrIf, br_if_body),
    ];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_medium_1br_table() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/medium_1br_table.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut block_body = vec![];
    block_body.push(Operator::I32Const { value: 12 });
    block_body.push(Operator::Drop);

    let mut br_table_body = vec![];
    br_table_body.push(Operator::I32Const { value: 23 });
    br_table_body.push(Operator::Drop);

    let ops_of_interest = vec![
        (SupportedOperators::Block, block_body),
        (SupportedOperators::BrTable, br_table_body),
    ];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_medium_2br() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/medium_2br.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut block_body = vec![];
    block_body.push(Operator::I32Const { value: 12 });
    block_body.push(Operator::Drop);

    let mut br_body = vec![];
    br_body.push(Operator::I32Const { value: 23 });
    br_body.push(Operator::Drop);

    let ops_of_interest = vec![
        (SupportedOperators::Block, block_body),
        (SupportedOperators::Br, br_body),
    ];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_medium_2br_if() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/medium_2br_if.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut block_body = vec![];
    block_body.push(Operator::I32Const { value: 12 });
    block_body.push(Operator::Drop);

    let mut br_if_body = vec![];
    br_if_body.push(Operator::I32Const { value: 23 });
    br_if_body.push(Operator::Drop);

    let ops_of_interest = vec![
        (SupportedOperators::Block, block_body),
        (SupportedOperators::BrIf, br_if_body),
    ];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_medium_2br_table() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/medium_2br_table.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut block_body = vec![];
    block_body.push(Operator::I32Const { value: 12 });
    block_body.push(Operator::Drop);

    let mut br_table_body = vec![];
    br_table_body.push(Operator::I32Const { value: 23 });
    br_table_body.push(Operator::Drop);

    let ops_of_interest = vec![
        (SupportedOperators::Block, block_body),
        (SupportedOperators::BrTable, br_table_body),
    ];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_medium_blocks() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/medium_blocks.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut block_body = vec![];
    block_body.push(Operator::I32Const { value: 12 });
    block_body.push(Operator::Drop);

    let mut br_table_body = vec![];
    br_table_body.push(Operator::I32Const { value: 34 });
    br_table_body.push(Operator::Drop);

    let ops_of_interest = vec![
        (SupportedOperators::Block, block_body),
        (SupportedOperators::BrTable, br_table_body),
    ];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_medium_ifelse() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/medium_ifelse.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut block_body = vec![];
    block_body.push(Operator::I32Const { value: 12 });
    block_body.push(Operator::Drop);

    let mut if_body = vec![];
    if_body.push(Operator::I32Const { value: 23 });
    if_body.push(Operator::Drop);

    let mut else_body = vec![];
    else_body.push(Operator::I32Const { value: 34 });
    else_body.push(Operator::Drop);

    let mut br_body = vec![];
    br_body.push(Operator::I32Const { value: 45 });
    br_body.push(Operator::Drop);

    let mut br_table_body = vec![];
    br_table_body.push(Operator::I32Const { value: 56 });
    br_table_body.push(Operator::Drop);

    let ops_of_interest = vec![
        (SupportedOperators::Block, block_body),
        (SupportedOperators::If, if_body),
        (SupportedOperators::Else, else_body),
        (SupportedOperators::Br, br_body),
        (SupportedOperators::BrTable, br_table_body),
    ];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_medium_ifs() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/medium_ifs.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut block_body = vec![];
    block_body.push(Operator::I32Const { value: 12 });
    block_body.push(Operator::Drop);

    let mut if_body = vec![];
    if_body.push(Operator::I32Const { value: 23 });
    if_body.push(Operator::Drop);

    let mut br_body = vec![];
    br_body.push(Operator::I32Const { value: 34 });
    br_body.push(Operator::Drop);

    let mut br_table_body = vec![];
    br_table_body.push(Operator::I32Const { value: 45 });
    br_table_body.push(Operator::Drop);

    let ops_of_interest = vec![
        (SupportedOperators::Block, block_body),
        (SupportedOperators::If, if_body),
        (SupportedOperators::Br, br_body),
        (SupportedOperators::BrTable, br_table_body),
    ];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_medium_multiple() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/medium_multiple.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut br_if_body = vec![];
    br_if_body.push(Operator::I32Const { value: 1234 });
    br_if_body.push(Operator::Drop);

    let mut br_table_body = vec![];
    br_table_body.push(Operator::I32Const { value: 5678 });
    br_table_body.push(Operator::Drop);

    let ops_of_interest = vec![
        (SupportedOperators::BrIf, br_if_body),
        (SupportedOperators::BrTable, br_table_body),
    ];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_medium_other_operators() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/medium_other_operators.wat";
    // todo -- test the other operators (when I know how to write wat using them)
    assert!(true);
}

#[test]
fn test_semantic_after_simple_1br() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/simple_1br.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut br_body = vec![];
    br_body.push(Operator::I32Const { value: 1234 });
    br_body.push(Operator::Drop);

    let ops_of_interest = vec![(SupportedOperators::Br, br_body)];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_simple_1br_if() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/simple_1br_if.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut br_if_body = vec![];
    br_if_body.push(Operator::I32Const { value: 1234 });
    br_if_body.push(Operator::Drop);

    let ops_of_interest = vec![(SupportedOperators::BrIf, br_if_body)];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_simple_1br_table() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/simple_1br_table.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut br_table_body = vec![];
    br_table_body.push(Operator::I32Const { value: 1234 });
    br_table_body.push(Operator::Drop);

    let ops_of_interest = vec![(SupportedOperators::BrTable, br_table_body)];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_simple_1if() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/simple_1if.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut if_body = vec![];
    if_body.push(Operator::I32Const { value: 12 });
    if_body.push(Operator::Drop);

    let mut br_body = vec![];
    br_body.push(Operator::I32Const { value: 34 });
    br_body.push(Operator::Drop);

    let ops_of_interest = vec![
        (SupportedOperators::If, if_body),
        (SupportedOperators::Br, br_body),
    ];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_simple_2br() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/simple_2br.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut br_body = vec![];
    br_body.push(Operator::I32Const { value: 1234 });
    br_body.push(Operator::Drop);

    let ops_of_interest = vec![(SupportedOperators::Br, br_body)];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_simple_2br_if() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/simple_2br_if.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut br_if_body = vec![];
    br_if_body.push(Operator::I32Const { value: 1234 });
    br_if_body.push(Operator::Drop);

    let ops_of_interest = vec![(SupportedOperators::BrIf, br_if_body)];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

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
fn test_semantic_after_simple_2br_table() {
    let file = "tests/test_inputs/instr_testing/modules/semantic_after/simple_2br_table.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let mut br_table_body = vec![];
    br_table_body.push(Operator::I32Const { value: 1234 });
    br_table_body.push(Operator::Drop);

    let ops_of_interest = vec![(SupportedOperators::BrTable, br_table_body)];
    inject_semantic_after(&mut mod_it, &ops_of_interest);

    let result = module.encode();
    let out = wasmprinter::print_bytes(result).expect("couldn't translate wasm to wat");
    if let Err(e) = check_instrumentation_encoding(&out, file) {
        error!(
            "Something went wrong when checking instrumentation encoding: {}",
            e
        )
    }
}

// =================
// ==== HELPERS ====
// =================

// This logic is necessary to avoid using `discriminant` fn for more complex enums
enum SupportedOperators {
    // block-style
    Block,
    Loop,
    If,
    Else,

    // branching
    Br,
    BrIf,
    BrTable,
    // TODO -- test the following
    // BrOnCast,
    // BrOnCastFail,
    // BrOnNull,
    // BrOnNonNull
}

fn inject_semantic_after<'a, 'b, 'c>(
    mod_it: &mut ModuleIterator<'a, 'b>,
    ops_of_interest: &Vec<(SupportedOperators, Vec<Operator<'c>>)>,
) where
    'c: 'b,
{
    loop {
        let op = mod_it.curr_op();
        if let Location::Module {
            func_idx,
            instr_idx,
        } = mod_it.curr_loc()
        {
            trace!("Func: {}, {}: {:?},", func_idx, instr_idx, op);

            for (op, body) in ops_of_interest.iter() {
                let matches = match op {
                    SupportedOperators::Block => {
                        matches!(mod_it.curr_op().unwrap(), Operator::Block { .. })
                    }
                    SupportedOperators::Loop => {
                        matches!(mod_it.curr_op().unwrap(), Operator::Loop { .. })
                    }
                    SupportedOperators::If => {
                        matches!(mod_it.curr_op().unwrap(), Operator::If { .. })
                    }
                    SupportedOperators::Else => {
                        matches!(mod_it.curr_op().unwrap(), Operator::Else { .. })
                    }
                    SupportedOperators::Br => {
                        matches!(mod_it.curr_op().unwrap(), Operator::Br { .. })
                    }
                    SupportedOperators::BrIf => {
                        matches!(mod_it.curr_op().unwrap(), Operator::BrIf { .. })
                    }
                    SupportedOperators::BrTable => {
                        matches!(mod_it.curr_op().unwrap(), Operator::BrTable { .. })
                    } // SupportedOperators::BrOnCast => matches!(mod_it.curr_op().unwrap(), Operator::BrOnCast {..}),
                      // SupportedOperators::BrOnCastFail => matches!(mod_it.curr_op().unwrap(), Operator::BrOnCastFail {..}),
                      // SupportedOperators::BrOnNull => matches!(mod_it.curr_op().unwrap(), Operator::BrOnNull {..}),
                      // SupportedOperators::BrOnNonNull => matches!(mod_it.curr_op().unwrap(), Operator::BrOnNonNull {..}),
                };
                if matches {
                    mod_it.semantic_after();
                    for op in body.iter() {
                        mod_it.inject(op.clone());
                    }
                }
            }

            if mod_it.next().is_none() {
                break;
            };
        } else {
            panic!("Should've gotten Module Location!");
        }
    }
}
