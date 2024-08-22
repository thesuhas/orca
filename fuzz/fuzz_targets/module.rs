// fuzzing roundtrips for modules

#![no_main]

use libfuzzer_sys::fuzz_target;
use wasm_smith::Module;
use orca;

fuzz_target!(|module: Module| {
    let wasm_bytes = module.to_bytes();
    // write to file
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::create("temp.wat").unwrap();
    
    let original = wasmprinter::print_bytes(wasm_bytes.clone()).expect("couldn't convert original Wasm to wat");
    file.write_all(&original.clone().into_bytes()).unwrap();
    // println!("original:\n {:?}", original);

    let mut wasm_module = orca::Module::parse(&wasm_bytes, false).unwrap();
    let roundtrip_bytes = wasm_module.encode();

    let roundtrip = wasmprinter::print_bytes(roundtrip_bytes.clone()).expect("couldn't convert roundtrip Wasm to wat");
    // println!("roundtrip:\n {:?}", roundtrip);

    assert_eq!(original, roundtrip);
});
