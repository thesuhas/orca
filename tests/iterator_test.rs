use orca::ir::component::Component;
use orca::ir::module::Module;
use orca::ir::types::{InstrumentType, Location};
use orca::iterator::component_iterator::ComponentIterator;
use orca::iterator::iterator_trait::Iterator;
use orca::iterator::module_iterator::ModuleIterator;
use orca::opcode::Opcode;
use orca::ModuleBuilder;
use std::collections::{HashMap, HashSet};
use wasmparser::Operator;

#[test]
fn test_iterator_count() {
    let mut count = 0;
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&mut component, HashMap::new());

    loop {
        let op = comp_it.curr_op();
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = comp_it.curr_loc()
        {
            println!(
                "Mod: {}, Fun: {}, {}: {:?},",
                mod_idx, func_idx, instr_idx, op
            );
            count += 1;
            if comp_it.next().is_none() {
                break;
            };
        } else {
            panic!("Should've gotten Component Location!");
        }
    }

    assert_eq!(count, 10);
}

#[test]
fn test_iterator_count_mul_mod() {
    let mut count = 0;
    let file = "tests/handwritten/components/mul_mod.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&mut component, HashMap::new());

    loop {
        let op = comp_it.curr_op();
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = comp_it.curr_loc()
        {
            println!(
                "Mod: {}, Fun: {}, {}: {:?},",
                mod_idx, func_idx, instr_idx, op
            );
            count += 1;
            if comp_it.next().is_none() {
                break;
            };
        } else {
            panic!("Should've gotten Component Location!");
        }
    }
    assert_eq!(count, 15);
}

// example of a ModuleIterator
#[test]
fn test_blocks() {
    let file = "tests/handwritten/modules/blocks.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    loop {
        let op = mod_it.curr_op();
        if let Location::Module {
            func_idx,
            instr_idx,
        } = mod_it.curr_loc()
        {
            println!("Fun: {}, {}: {:?},", func_idx, instr_idx, op);
            if mod_it.next().is_none() {
                break;
            };
        } else {
            panic!("Should've gotten Component Location!");
        }
    }
}

#[test]
fn iterator_mark_as_before_test() {
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&mut component, HashMap::new());

    let interested = Operator::Call { function_index: 1 };

    loop {
        let op = comp_it.curr_op();
        let instr_type = comp_it.curr_instrument_type();

        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = comp_it.curr_loc()
        {
            println!(
                "Mod: {}, Fun: {}, +{}: {:?}, {:?}",
                mod_idx, func_idx, instr_idx, op, instr_type
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
        let instr_type = comp_it.curr_instrument_type();
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = comp_it.curr_loc()
        {
            if *comp_it.curr_op().unwrap() == interested {
                assert_ne!(instr_type, InstrumentType::NotInstrumented);
            }

            println!(
                "Mod: {}, Fun: {}, +{}: {:?}, {:?}",
                mod_idx, func_idx, instr_idx, op, instr_type
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
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&mut component, HashMap::new());

    let interested = Operator::Call { function_index: 1 };

    loop {
        let op = comp_it.curr_op();
        let instr_type = comp_it.curr_instrument_type();

        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = comp_it.curr_loc()
        {
            println!(
                "Mod: {}, Fun: {}, +{}: {:?}, {:?}",
                mod_idx, func_idx, instr_idx, op, instr_type
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

    println!("\nAfter instrumentation\n");

    loop {
        let op = comp_it.curr_op();
        let instr_type = comp_it.curr_instrument_type();

        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = comp_it.curr_loc()
        {
            println!(
                "Mod: {}, Fun: {}, +{}: {:?}, {:?}",
                mod_idx, func_idx, instr_idx, op, instr_type
            );
            if *comp_it.curr_op().unwrap() == interested {
                assert_eq!(
                    comp_it.curr_instrument_type(),
                    InstrumentType::InstrumentBefore
                );
                assert_eq!(
                    comp_it.get_injected_val(0),
                    &Operator::I32Const { value: 1 }
                );
            }
            if comp_it.next().is_none() {
                break;
            };
        } else {
            panic!("Should've gotten Component Location!");
        }
    }
}

// you can also inline this
fn iterate(component: &mut Component) {
    let mut comp_it = ComponentIterator::new(component, HashMap::new());

    let after = Operator::Call { function_index: 1 };
    let before = Operator::Drop;
    let alternate = Operator::I32Const { value: 2 };

    loop {
        let op = comp_it.curr_op();
        let instr_type = comp_it.curr_instrument_type();

        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = comp_it.curr_loc()
        {
            println!(
                "Mod: {}, Fun: {}, +{}: {:?}, {:?}",
                mod_idx, func_idx, instr_idx, op, instr_type
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
}

#[test]
// TODO: no assertions for now, verify by eyeballing
fn iterator_verify_injection() {
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");

    iterate(&mut component);

    let result = component.encode();
    let out = wasmprinter::print_bytes(result).expect("couldn't translated Wasm to wat");

    println!("{}", out);
}

// example of a adding locals via an iterator
// TODO: no assertions for now, verify by eyeballing
#[test]
fn test_it_add_local() {
    let file = "tests/handwritten/modules/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    loop {
        let op = mod_it.curr_op();
        if let Location::Module {
            func_idx,
            instr_idx,
        } = mod_it.curr_loc()
        {
            println!("Fun: {}, {}: {:?},", func_idx, instr_idx, op);

            if mod_it.curr_op().unwrap() == &Operator::I32Add {
                let local_id = mod_it.add_local(orca::ir::types::DataType::I32);
                println!("new Local ID: {:?}", local_id);
            }

            if mod_it.curr_op().unwrap() == &(Operator::I32Const { value: 2 }) {
                let local_id = mod_it.add_local(orca::ir::types::DataType::I32);
                println!("new Local ID: {:?}", local_id);
            }

            if mod_it.next().is_none() {
                break;
            };
        } else {
            panic!("Should've gotten Component Location!");
        }
    }

    let a = module.encode();
    let wat = wasmprinter::print_bytes(&a).unwrap();
    println!("{}", wat);
}

// example of splicing an instrument at specific location
// TODO: no assertions for now, verify by eyeballing
#[test]
fn test_it_instr_at() {
    let file = "tests/handwritten/modules/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    let loc = Location::Module {
        func_idx: 0,
        instr_idx: 1,
    };
    mod_it.before_at(loc.clone());
    mod_it.add_instr_at(loc, Operator::Unreachable);
    loop {
        let op = mod_it.curr_op();
        if let Location::Module {
            func_idx,
            instr_idx,
        } = mod_it.curr_loc()
        {
            println!("Fun: {}, {}: {:?},", func_idx, instr_idx, op);
        } else {
            panic!("Should've gotten Component Location!");
        }
        if mod_it.next().is_none() {
            break;
        };
    }

    let a = module.encode();
    let wat = wasmprinter::print_bytes(&a).unwrap();
    println!("{}", wat);
}

// example of duplicating every instruction
// TODO: no assertions for now, verify by eyeballing
#[test]
fn test_it_dup_instr() {
    let file = "tests/handwritten/modules/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    loop {
        let op = mod_it.curr_op();
        if let Location::Module {
            func_idx,
            instr_idx,
        } = mod_it.curr_loc()
        {
            println!("Fun: {}, {}: {:?},", func_idx, instr_idx, op);

            let loc = mod_it.curr_loc();
            let orig = mod_it.curr_op_owned().unwrap();
            mod_it.before();
            mod_it.add_instr_at(loc, orig);
        } else {
            panic!("Should've gotten Component Location!");
        }
        if mod_it.next().is_none() {
            break;
        };
    }

    let a = module.encode();
    let wat = wasmprinter::print_bytes(&a).unwrap();
    println!("{}", wat);
}

// no asserts, eyeballing for now
#[test]
fn test_imports() {
    let file = "tests/handwritten/modules/import.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    // println!("{:#?}", module);

    let num_imported_func = module.num_import_func();
    assert_eq!(num_imported_func, 2);

    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    loop {
        let op = mod_it.curr_op();
        if let Location::Module {
            func_idx,
            instr_idx,
        } = mod_it.curr_loc()
        {
            println!(
                "Fun: {}, {}: {:?},",
                num_imported_func as usize + func_idx,
                instr_idx,
                op
            );
        } else {
            panic!("Should've gotten Component Location!");
        }
        if mod_it.next().is_none() {
            break;
        };
    }

    let a = module.encode();
    let wat = wasmprinter::print_bytes(&a).unwrap();
    println!("{}", wat);
}

#[test]
fn test_it_add_local_diff_type() {
    let file = "tests/handwritten/modules/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let mut mod_it = ModuleIterator::new(&mut module, vec![]);

    mod_it.add_local(orca::ir::types::DataType::I64);
    mod_it.add_local(orca::ir::types::DataType::I32);
    let a = module.encode();
    let wat = wasmprinter::print_bytes(&a).unwrap();
    println!("{}", wat);
}

#[test]
fn test_function_skipping_module() {
    let file = "tests/handwritten/modules/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    let functions_skip = vec![0usize];
    let mut mod_it = ModuleIterator::new(&mut module, functions_skip);

    let mut set = HashSet::new();

    loop {
        if let Location::Module {
            func_idx,
            instr_idx: _instr_idx,
        } = mod_it.curr_loc()
        {
            set.insert(func_idx);
        } else {
            panic!("Should've gotten Module Location!");
        }
        if mod_it.next().is_none() {
            break;
        };
    }

    assert_eq!(set.len(), 1);
    assert!(set.contains(&1usize));
}

#[test]
fn test_function_skipping_component() {
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut comp = Component::parse(&buff, false).expect("Unable to parse");
    let functions_skip = vec![0usize];
    let mut mapping = HashMap::new();
    mapping.insert(0usize, functions_skip);
    let mut comp_it = ComponentIterator::new(&mut comp, mapping);

    let mut set = HashSet::new();

    loop {
        if let Location::Component {
            mod_idx: _mod_idx,
            func_idx,
            instr_idx: _instr_idx,
        } = comp_it.curr_loc()
        {
            set.insert(func_idx);
        } else {
            panic!("Should've gotten Component Location!");
        }
        if comp_it.next().is_none() {
            break;
        };
    }

    assert_eq!(set.len(), 1);
    assert!(set.contains(&1usize));
}

// #[test]
// fn test_fn_name() {
//     let file = "tests/handwritten/modules/add.wat";
//
//     let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
//     let module = Module::parse(&buff, false).expect("Unable to parse");
//     assert_eq!("add".to_string(), module.get_fname(0).unwrap());
//     assert_eq!(None, module.get_fname(1));
// }
