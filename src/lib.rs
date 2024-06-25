mod convert;
mod error;
pub mod ir;

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::ir::Component;
    use std::path::Path;

    #[test]
    fn parse() {
        let path = Path::new("test/hello_world.wasm");
        let data = fs::read(path).expect("File not found");
        let cmp = Component::parse(&data, false).expect("TODO: panic message");
        println!("aha");
    }
}