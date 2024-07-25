use orca::ir::module::Module;

fn round_trip_module(testname: &str, folder: &str) {
    let filename = format!(
        "{}/tests/{}/{}.wat",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        folder,
        testname
    );
    let buff = wat::parse_file(filename).expect("couldn't convert the input wat to Wasm");

    let module = Module::parse(&buff, false).unwrap();
    let result = module.encode();
    let out = wasmprinter::print_bytes(result).expect("couldn't translated Wasm to wat");
    println!("{}", out);
    // let original = wasmprinter::print_bytes(buff).expect("couldn't convert original Wasm to wat");
    // assert_eq!(out, original);
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

    make_round_trip_tests_module!("handwritten/modules", blocks);
}

#[test]
fn set_name() {
    let filename = "tests/handwritten/modules/func1.wat";
    let buff = wat::parse_file(filename).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).unwrap();
    module.set_fn_name(1, "test");
    println!("{:#?}", module);
    let result = module.encode();
    //write result to file
    let filename = "a.wasm";
    let mut file = std::fs::File::create(filename).expect("couldn't create file");
    use std::io::Write;
    file.write_all(&result).expect("couldn't write to file");
    let out = wasmprinter::print_bytes(result).expect("couldn't translated Wasm to wat");
    println!("{}", out);
}
