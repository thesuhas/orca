use std::fs::File;
use std::process::{Command, Stdio};

#[test]
// build factorial from scratch
fn run_fac_orca() {
    // run cargo run in fac_orca dir
    let a = Command::new("cargo")
        .arg("run")
        .current_dir("fac_orca")
        .output()
        .expect("failed to execute process");
    assert!(a.status.success());

    let fac_generated = wasmprinter::print_file("fac_orca/target/out.wasm").unwrap();
    let fac_standard = wasmprinter::print_file("fac_orca/fact.wasm").unwrap();
    assert_eq!(fac_generated, fac_standard);
}
