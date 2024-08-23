use gimli::{read, Dwarf};

/// The DWARF debug section in input WebAssembly binary.
#[derive(Debug, Default, Clone)]
pub struct ModuleDebugData {
    /// DWARF debug data
    pub dwarf: DwarfData,
}

#[derive(Debug, Default)]
pub struct DwarfData(Dwarf<Vec<u8>>);

impl Clone for DwarfData {
    fn clone(&self) -> Self {
        let dwarf = &self.0;

        DwarfData(Dwarf {
            debug_abbrev: dwarf.debug_abbrev.clone(),
            debug_addr: dwarf.debug_addr.clone(),
            debug_aranges: dwarf.debug_aranges.clone(),
            debug_info: dwarf.debug_info.clone(),
            debug_line: dwarf.debug_line.clone(),
            debug_line_str: dwarf.debug_line_str.clone(),
            debug_str: dwarf.debug_str.clone(),
            debug_str_offsets: dwarf.debug_str_offsets.clone(),
            debug_types: dwarf.debug_types.clone(), // Clone the rest of the fields if needed
            locations: dwarf.locations.clone(),
            ranges: dwarf.ranges.clone(),
            file_type: dwarf.file_type.clone(),
            sup: dwarf.sup.clone(),
            abbreviations_cache: dwarf.abbreviations_cache.,
        })
    }
}