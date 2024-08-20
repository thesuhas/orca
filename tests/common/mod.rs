use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub const WASM_OUTPUT_DIR: &str = "output/wasm";
pub const WAT_OUTPUT_DIR: &str = "output/wat";

/// create output path if it doesn't exist
pub fn try_path(path: &String) {
    if !PathBuf::from(path).exists() {
        fs::create_dir_all(PathBuf::from(path).parent().unwrap()).unwrap();
    }
}

// ========================
// ==== TEST FRAMEWORK ====
// ========================

pub fn check_instrumentation_encoding(orca_wat: &String, file: &str) -> Result<(), std::io::Error> {
    let f = File::open(file)?;
    let mut reader = BufReader::new(f);
    let wat_with_instr = get_wat_with_inline_instrumentation(&mut reader)?;
    assert_eq!(*orca_wat, wat_with_instr);
    Ok(())
}

const INSERT_PREFIX_PATTERN: &str = ";; << ";
const REPLACE_PREFIX_PATTERN: &str = ";; < ";
const REMOVE_PREFIX_PATTERN: &str = ";; rm";
fn get_wat_with_inline_instrumentation(
    reader: &mut BufReader<File>,
) -> Result<String, std::io::Error> {
    let mut wat_with_instr = String::new();

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        if line.contains(REMOVE_PREFIX_PATTERN) {
            // Do not include
            line.clear();
            continue;
        } else if line.contains(INSERT_PREFIX_PATTERN) {
            // Just insert the code! This should retain indentation
            let new_line = line.replace(INSERT_PREFIX_PATTERN, "");
            wat_with_instr += &new_line;
        } else if line.contains(REPLACE_PREFIX_PATTERN) {
            // Replace the code! Just remove all non-whitespace before and the pattern itself

            // Find the end of the indentation
            let mut end_whitespace_idx = 0;
            for (idx, c) in line.chars().enumerate() {
                if !c.is_whitespace() {
                    end_whitespace_idx = idx;
                    break;
                }
            }
            // Find the beginning of the command
            let command_start = line.find(REPLACE_PREFIX_PATTERN).unwrap();

            // remove original
            line.replace_range(end_whitespace_idx..command_start, "");
            // remove the command
            let new_line = line.replace(REPLACE_PREFIX_PATTERN, "");
            wat_with_instr += &new_line;
        } else {
            wat_with_instr += &line;
        }

        line.clear();
    }
    Ok(
        wasmprinter::print_bytes(wat::parse_str(wat_with_instr.clone()).expect("Error encoding"))
            .unwrap(),
    )
}
