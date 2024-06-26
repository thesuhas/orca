mod convert;
mod error;
pub mod ir;
mod wrappers;

#[cfg(test)]
mod tests {
    use crate::ir::Component;
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn parse() {
        let path = Path::new("test/test_c.wasm");
        let data = fs::read(path).expect("File not found");
        let cmp = Component::parse(&data, false).expect("TODO: panic message");
        let out = cmp.encode().unwrap();
        let out_path = Path::new("test/out.wasm");
        let mut out_file = fs::File::create(out_path).expect("Unable to create file");
        out_file.write_all(&out).expect("Write failed");
        assert_eq!(out, data);
    }
}
