use gimli::read::{Dwarf};

/// The DWARF debug section in input WebAssembly binary.
#[derive(Debug, Default)]
pub struct ModuleDebugData {
    /// DWARF debug data
    pub dwarf: Dwarf<Vec<u8>>
}