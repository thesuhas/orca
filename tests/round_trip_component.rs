use orca::ir::Component;
use std::io::Write; // bring trait into scope
use std::fs::File;

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
    if out != original {
        println!("Test: {:?} failed! Writing to file to check", testname);

        let mut file = match File::create(format!("{}_test.wat", testname)) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to create the file: {}", e);
                return;
            },
        };

        // Write the string to the file
        match file.write_all(out.as_bytes()) {
            Ok(_) => println!("Data successfully written to the file."),
            Err(e) => eprintln!("Failed to write to the file: {}", e),
        }
    }
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

    make_round_trip_tests_component!(
        "handwritten/components",
        add
    );
}