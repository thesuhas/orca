use std::fs;
use std::path::PathBuf;

pub const WASM_OUTPUT_DIR: &str = "output/wasm";
pub const WAT_OUTPUT_DIR: &str = "output/wat";

/// create output path if it doesn't exist
pub fn try_path(path: &String) {
    if !PathBuf::from(path).exists() {
        fs::create_dir_all(PathBuf::from(path).parent().unwrap()).unwrap();
    }
}
