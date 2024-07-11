use orca::ir::component::Component;
use orca::ir::iterator::ComponentIterator;
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
        (Operator::I32Const { value: value1 }, Operator::I32Const { value: value2 }) => {
            value1 == value2
        }
        _ => false,
    }
}

#[test]
fn iterator_test() {
    let mut count = 0;
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&mut component);

    loop {
        match comp_it.next() {
            Some(_) => count += 1,
            None => break,
        }
    }
    assert_eq!(count, 10);
}

#[test]
fn iterator_mark_as_before_test() {
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let mut component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&mut component);

    let interested = Operator::Call { function_index: 1 };

    loop {
        match comp_it.next() {
            Some(op) => {
                if is_same_call(op, &interested) {
                    comp_it.before();
                }
            }
            None => break,
        }
    }

    comp_it.reset();

    loop {
        match comp_it.next() {
            Some(op) => {
                if is_same_call(op, &interested) {
                    assert_eq!(
                        *comp_it.get_instrument_type(),
                        InstrumentType::InstrumentBefore(vec![])
                    );
                }
            }
            None => break,
        }
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
        match comp_it.next() {
            Some(op) => {
                if is_same_call(op, &interested) {
                    comp_it.before();
                    comp_it.i32(0);
                }
            }
            None => break,
        }
    }

    comp_it.reset();

    loop {
        match comp_it.next() {
            Some(op) => {
                if is_same_call(op, &interested) {
                    assert_eq!(
                        *comp_it.get_instrument_type(),
                        InstrumentType::InstrumentBefore(vec![])
                    );
                    assert_eq!(
                        is_same_call(
                            comp_it.get_injected_val(0),
                            &Operator::I32Const { value: 0 }
                        ),
                        true
                    );
                }
            }
            None => break,
        }
    }
}
