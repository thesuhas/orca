use orca::ir::component::Component;
use orca::ir::iterator::ComponentIterator;
use orca::ir::module::Module;
use orca::ir::types::InstrumentType;
use std::fs::File;
use std::io::Write;
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
        (Operator::I32Const { value: value1 }, Operator::I32Const { value: value2 }) => {
            value1 == value2
        }
        _ => false,
    }
}

#[test]
fn test_iterator_count() {
    let mut count = 0;
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&mut component);

    loop {
        let op = comp_it.curr_op();
        let mod_idx = comp_it.curr_mod_idx();
        let fun_idx = comp_it.curr_func_idx();
        let instr_idx = comp_it.curr_instr_idx();
        println!(
            "Mod: {}, Fun: {}, {}: {:?},",
            mod_idx, fun_idx, instr_idx, op
        );
        count += 1;
        if comp_it.next().is_none() {
            break;
        };
    }

    assert_eq!(count, 10);
}

#[test]
fn test_iterator_count_mul_mod() {
    let mut count = 0;
    let file = "tests/handwritten/components/mul_mod.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&mut component);

    loop {
        let op = comp_it.curr_op();
        let mod_idx = comp_it.curr_mod_idx();
        let fun_idx = comp_it.curr_func_idx();
        let instr_idx = comp_it.curr_instr_idx();
        println!(
            "Mod: {}, Fun: {}, {}: {:?},",
            mod_idx, fun_idx, instr_idx, op
        );
        count += 1;
        if comp_it.next().is_none() {
            break;
        };
    }
    assert_eq!(count, 15);
}

fn module_to_component(module: Module) -> Component {
    let mut component = Component::new();
    component.add_module(module);
    component
}

#[test]
fn test_blocks() {
    let file = "tests/handwritten/modules/blocks.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let module = Module::parse_only_module(&buff, false).expect("Unable to parse");
    let mut component = module_to_component(module);
    let mut comp_it = ComponentIterator::new(&mut component);

    loop {
        let op = comp_it.curr_op();
        let mod_idx = comp_it.curr_mod_idx();
        let fun_idx = comp_it.curr_func_idx();
        let instr_idx = comp_it.curr_instr_idx();
        println!(
            "Mod: {}, Fun: {}, {}: {:?},",
            mod_idx, fun_idx, instr_idx, op
        );
        if comp_it.next().is_none() {
            break;
        };
    }
}

#[test]
fn iterator_mark_as_before_test() {
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&mut component);

    let interested = Operator::Call { function_index: 1 };

    loop {
        let op = comp_it.curr_op();
        let mod_idx = comp_it.curr_mod_idx();
        let fun_idx = comp_it.curr_func_idx();
        let instr_idx = comp_it.curr_instr_idx();
        let instr_type = comp_it.get_instrument_type();
        println!(
            "Mod: {}, Fun: {}, +{}: {:?}, {:?}",
            mod_idx, fun_idx, instr_idx, op, instr_type
        );
        if is_same_call(comp_it.curr_op().unwrap(), &interested) {
            comp_it.before();
        }
        if comp_it.next().is_none() {
            break;
        };
    }

    comp_it.reset();

    loop {
        let op = comp_it.curr_op();
        let mod_idx = comp_it.curr_mod_idx();
        let fun_idx = comp_it.curr_func_idx();
        let instr_idx = comp_it.curr_instr_idx();
        let instr_type = comp_it.get_instrument_type();
        if is_same_call(comp_it.curr_op().unwrap(), &interested) {
            assert_ne!(*instr_type, InstrumentType::NotInstrumented);
        }

        println!(
            "Mod: {}, Fun: {}, +{}: {:?}, {:?}",
            mod_idx, fun_idx, instr_idx, op, instr_type
        );

        if comp_it.next().is_none() {
            break;
        };
    }
}

#[test]
fn iterator_inject_i32_before() {
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&mut component);

    let interested = Operator::Call { function_index: 1 };

    loop {
        let op = comp_it.curr_op();
        let mod_idx = comp_it.curr_mod_idx();
        let fun_idx = comp_it.curr_func_idx();
        let instr_idx = comp_it.curr_instr_idx();
        let instr_type = comp_it.get_instrument_type();
        println!(
            "Mod: {}, Fun: {}, +{}: {:?}, {:?}",
            mod_idx, fun_idx, instr_idx, op, instr_type
        );
        if is_same_call(comp_it.curr_op().unwrap(), &interested) {
            comp_it.before().i32(1);
        }
        if comp_it.next().is_none() {
            break;
        };
    }

    comp_it.reset();

    println!("\nAfter instrumentation\n");

    loop {
        let op = comp_it.curr_op();
        let mod_idx = comp_it.curr_mod_idx();
        let fun_idx = comp_it.curr_func_idx();
        let instr_idx = comp_it.curr_instr_idx();
        let instr_type = comp_it.get_instrument_type();
        println!(
            "Mod: {}, Fun: {}, +{}: {:?}, {:?}",
            mod_idx, fun_idx, instr_idx, op, instr_type
        );
        if is_same_call(comp_it.curr_op().unwrap(), &interested) {
            assert_eq!(
                *comp_it.get_instrument_type(),
                InstrumentType::InstrumentBefore(vec![])
            );
            assert_eq!(
                is_same_call(
                    comp_it.get_injected_val(0),
                    &Operator::I32Const { value: 1 }
                ),
                true
            );
        }
        if comp_it.next().is_none() {
            break;
        };
    }
}

#[test]
fn iterator_verify_injection() {
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&mut component);

    let interested = Operator::Call { function_index: 1 };

    loop {
        let op = comp_it.curr_op();
        let mod_idx = comp_it.curr_mod_idx();
        let fun_idx = comp_it.curr_func_idx();
        let instr_idx = comp_it.curr_instr_idx();
        let instr_type = comp_it.get_instrument_type();
        println!(
            "Mod: {}, Fun: {}, +{}: {:?}, {:?}",
            mod_idx, fun_idx, instr_idx, op, instr_type
        );
        if is_same_call(comp_it.curr_op().unwrap(), &interested) {
            comp_it.before().i32(0);
        }
        if comp_it.next().is_none() {
            break;
        };
    }
    let comp = comp_it.get_component();
    println!("{:?}", comp);
    let result = comp.encode().expect("Error in Encoding");
    let out = wasmprinter::print_bytes(result).expect("couldn't translated Wasm to wat");

    let mut file = match File::create(format!("{}_test.wat", "add_test")) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create the file: {}", e);
            return;
        }
    };

    // Write the string to the file
    match file.write_all(out.as_bytes()) {
        Ok(_) => println!("Data successfully written to the file."),
        Err(e) => eprintln!("Failed to write to the file: {}", e),
    }
}
