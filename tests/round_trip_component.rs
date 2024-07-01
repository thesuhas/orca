use orca::ir::Component;

fn round_trip_component(testname: &str, folder: &str) {
    let filename = format!(
        "{}/tests/{}/{}.wat",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        folder,
        testname
    );
    let buff = wat::parse_file(filename).expect("couldn't convert the input wat to Wasm");

    let component = Component::parse(&buff, false).expect("Unable to parse");
    let result = component.encode().expect("Unable to encode");
    let out = wasmprinter::print_bytes(result).expect("couldn't translated Wasm to wat");
    let original = wasmprinter::print_bytes(buff).expect("couldn't convert original Wasm to wat");
    assert_eq!(out, original);
}

macro_rules! make_round_trip_tests_component {
    ($folder:literal, $($name:ident),*) => {
        $(
            #[test]
            fn $name() {
                crate::round_trip_component(stringify!($name), $folder)
            }
        )*
    };
}

mod round_trip {
    make_round_trip_tests_component!(
        "dfinity/components",
        data_section,
        func,
        func_locals,
        table,
        table_init,
        exports,
        start,
        const_expr
    );

}
