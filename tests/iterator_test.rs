use orca::ir::component::Component;
use orca::ir::iterator::ComponentIterator;
use orca::ir::module::Module;
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

// TODO: add asserts, I currently test this with eyeballing
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
            "Mod: {}, Fun: {}, {}: {:?}, {:?}",
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
        println!(
            "Mod: {}, Fun: {}, {}: {:?}, {:?}",
            mod_idx, fun_idx, instr_idx, op, instr_type
        );

        if comp_it.next().is_none() {
            break;
        };
    }
}
