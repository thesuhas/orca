// fuzzing roundtrips for component
#![no_main]

use libfuzzer_sys::fuzz_target;
use wasm_smith::Component;
use orca;

fuzz_target!(|comp: Component| {
    let wasm_bytes = comp.to_bytes();
    // write to file
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::create("temp.wat").unwrap();
    
    let original = wasmprinter::print_bytes(wasm_bytes.clone()).expect("couldn't convert original Wasm to wat");
    file.write_all(&original.clone().into_bytes()).unwrap();
    // println!("original:\n {:?}", original);

    let mut wasm_comp = orca::Component::parse(&wasm_bytes, false).unwrap();
    let roundtrip_bytes = wasm_comp.encode();

    let roundtrip = wasmprinter::print_bytes(roundtrip_bytes.clone()).expect("couldn't convert roundtrip Wasm to wat");
    // println!("roundtrip:\n {:?}", roundtrip);

    assert_eq!(original, roundtrip);
});
