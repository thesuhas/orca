use orca::ir::component::Component;
use std::fs::File;
use std::io::Write; // bring trait into scope

fn write_to_file(bytes: &Vec<u8>, path: String) {
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create the file: {}", e);
            return;
        }
    };

    // Write the string to the file
    match file.write_all(bytes) {
        Ok(_) => println!("Data successfully written to the file."),
        Err(e) => eprintln!("Failed to write to the file: {}", e),
    }
}

fn round_trip_component(testname: &str, folder: &str) {
    let filename = format!(
        "{}/tests/{}/{}.wat",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        folder,
        testname
    );
    let buff = wat::parse_file(filename).expect("couldn't convert the input wat to Wasm");

    let component = Component::parse(&buff, false).expect("Unable to parse");
    component.print();
    let result = component.encode();
    write_to_file(&result, format!("{}_test.wasm", testname));
    let out = wasmprinter::print_bytes(result).expect("couldn't translate Wasm to wat");
    let original = wasmprinter::print_bytes(buff).expect("couldn't convert original Wasm to wat");
    if out != original {
        println!("Test: {:?} failed! Writing to file to check", testname);

        let mut file = match File::create(format!("{}_test.wat", testname)) {
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
    assert_eq!(out, original);
    // println!("{}", out);
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
    // note, the dfinity test suite has name conflicts with wasm-tools
    // make_round_trip_tests_component!(
    //     "dfinity/components",
    //     data_section,
    //     func,
    //     func_locals,
    //     table,
    //     table_init,
    //     exports,
    //     start,
    //     const_expr
    // );

    // make_round_trip_tests_component!("handwritten/components", add);
    //
    // make_round_trip_tests_component!("wizard/components", func_loop);
    //
    // make_round_trip_tests_component!("spec-test/components", if_test);

    // make_round_trip_tests_component!("spin", hello_world);

    // TODO: fix me
    make_round_trip_tests_component!(
        "wasm-tools/component-model",
        //a,
        big // TODO: couldn't translate Wasm to wat: section size mismatch: unexpected data at the end of the section (at offset 0xd6)
            // link
            // lots_of_aliases,
            // more_flags, // passes!
            // string // TODO: section size mismatch: unexpected data at the end of the section (at offset 0xd9)
    );
}
