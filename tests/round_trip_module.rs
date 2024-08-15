mod common;

use crate::common::{try_path, WASM_OUTPUT_DIR, WAT_OUTPUT_DIR};
use log::{debug, error, trace};
use orca::ir::module::Module;
use std::fs::File;
use std::io::Write;

fn write_to_file(bytes: &Vec<u8>, path: String) {
    try_path(&path);
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to create the file: {}", e);
            return;
        }
    };

    // Write the string to the file
    match file.write_all(bytes) {
        Ok(_) => trace!("Data successfully written to the file."),
        Err(e) => error!("Failed to write to the file: {}", e),
    }
}

fn round_trip_module(testname: &str, folder: &str) {
    let filename = format!(
        "{}/tests/test_inputs/{}/{}.wat",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        folder,
        testname
    );
    let buff = wat::parse_file(filename).expect("couldn't convert the input wat to Wasm");
    let original =
        wasmprinter::print_bytes(buff.clone()).expect("couldn't convert original Wasm to wat");
    let mut module = Module::parse(&buff, false).unwrap();
    let result = module.encode();
    let out = wasmprinter::print_bytes(result).expect("couldn't translated Wasm to wat");

    if out != original {
        debug!("Test: {:?} failed! Writing to file to check", testname);
        write_to_file(
            &out.as_bytes().to_vec(),
            format!("{WAT_OUTPUT_DIR}/module_{}.wat", testname),
        );
    }
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
    // TODO -- change this to just search and find files (keeps from having to manually maintain this list)
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

    make_round_trip_tests_module!("handwritten/modules", add);
    make_round_trip_tests_module!(
        "handwritten/modules/blocks",
        complex_mult_nested_diff_opcodes,
        medium_1br,
        medium_1br_if,
        medium_1br_table,
        medium_2br,
        medium_2br_if,
        medium_2br_table,
        medium_ifelse,
        medium_ifs,
        medium_multiple,
        simple_1br,
        simple_1br_if,
        simple_1br_table,
        simple_2br,
        simple_2br_if,
        simple_2br_table
    );

    make_round_trip_tests_module!("spin", hello_world_module);
}

#[test]
fn set_name() {
    let filename = "tests/test_inputs/handwritten/modules/func1.wat";
    let buff = wat::parse_file(filename).expect("couldn't convert the input wat to Wasm");
    let mut module = Module::parse(&buff, false).unwrap();
    module.set_fn_name(1, "test".to_string());
    // println!("{:#?}", module);
    let result = module.encode();

    //write result to file
    write_to_file(&result, format!("{WASM_OUTPUT_DIR}/func1.wasm"));
    let out = wasmprinter::print_bytes(result).expect("couldn't translated Wasm to wat");
    debug!("{}", out);
}
