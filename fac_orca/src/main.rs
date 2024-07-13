use orca::ir::module::*;
use wasmparser::ValType;

fn main() {

    let mut module = Module::new();
    // let log_type_id = 
    let _ = module.add_type(&[ValType::I32], &[]);

    // module.emit_wasm("target/out.wasm").unwrap();
    let bytes = module.encode_only_module();
    let wat = wasmprinter::print_bytes(&bytes).unwrap();
    println!("{}", wat);
}
