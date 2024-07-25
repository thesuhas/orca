use std::fs::File;
use std::io::Write;
use orca::ir::module::Module;

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
    let original = wasmprinter::print_bytes(buff).expect("couldn't convert original Wasm to wat");
    if out != original {
        println!("Test: {:?} failed! Writing to file to check", testname);

        write_to_file(&out.as_bytes().to_vec(), format!("{}_test.wat", testname));
    }
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
    //     import_func,
    //     data_section,
    //     func,
    //     func_locals,
    //     table,
    //     table_init,
        globals
        // exports,
        // start,
        // const_expr
    );

    // make_round_trip_tests_module!("handwritten/modules", blocks);
    //
    // make_round_trip_tests_module!("spin", hello_world_module);
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
