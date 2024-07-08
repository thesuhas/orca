use orca::ir::module::Module;

fn round_trip_module(testname: &str, folder: &str) {
    let filename = format!(
        "{}/tests/{}/{}.wat",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        folder,
        testname
    );
    let buff = wat::parse_file(filename).expect("couldn't convert the input wat to Wasm");

    let module = Module::parse_only_module(&buff, false).unwrap();
    let result = module.encode_only_module().unwrap();
    let out = wasmprinter::print_bytes(result).expect("couldn't translated Wasm to wat");
    let original = wasmprinter::print_bytes(buff).expect("couldn't convert original Wasm to wat");
    assert_eq!(out, original);
}

macro_rules! make_round_trip_tests_module {
    ($folder:literal, $($name:ident),*) => {
        $(
            #[test]
            fn $name() {
                crate::round_trip_module(stringify!($name), $folder)
            }
        )*
    };
}

mod round_trip {
    make_round_trip_tests_module!(
        "dfinity/modules",
        import_func,
        data_section,
        func,
        func_locals,
        table,
        table_init,
        globals,
        exports,
        start,
        const_expr
    );
}
