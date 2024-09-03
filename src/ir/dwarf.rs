use gimli::read::Dwarf;
use gimli::{EndianSlice, LittleEndian};

/// The DWARF debug section in input WebAssembly binary.
#[derive(Debug, Default)]
pub struct ModuleDebugData<'a> {
    /// DWARF debug data
    pub dwarf: Dwarf<EndianSlice<'a, LittleEndian>>,
}
