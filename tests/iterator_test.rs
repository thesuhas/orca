use orca::ir::component::Component;
use orca::ir::iterator::ComponentIterator;

#[test]
fn iterator_test() {
    let mut count = 0;
    let file = "tests/handwritten/components/add.wat";

    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let component = Component::parse(&buff, false).expect("Unable to parse");
    let mut comp_it = ComponentIterator::new(&component);

    loop {
        match comp_it.next() {
            Some(_) => count += 1,
            None => break,
        }
    }
    assert_eq!(count, 8);
}
