use gimli::read::Dwarf;
use gimli::write::{ConvertError, EndianVec, LineProgram, LineString, Sections};
use gimli::{
    read, write, AttributeValue, EndianSlice, IncompleteLineProgram, LittleEndian, Reader,
    SectionId, Unit,
};
use std::collections::HashMap;

/// The DWARF debug section in input WebAssembly binary.
#[derive(Debug, Default)]
pub struct ModuleDebugData<'a> {
    /// DWARF debug data
    pub dwarf: Dwarf<EndianSlice<'a, LittleEndian>>,
    pub(crate) strings: write::StringTable,
    pub(crate) line_strings: write::LineStringTable,
}

pub(crate) static DEAD_CODE: u64 = 0xFFFFFFFF;

// .debug_line section
// Debug line section contains mapping from source code to offset in machine instructions, helps debugger associate source file
// with machine instructions. We need to update the source code mapping as on instrumentation the location would have changed.
// This section involves a Line Program Header and Line Program underneath.
// IMPORTANT NOTE: Lines are numbered beginning at 1. 0 is used when an instruction cannot be mapped to a source file

impl<'a> ModuleDebugData<'a> {
    pub fn convert(
        &mut self,
        instr_map: HashMap<usize, usize>,
    ) -> Sections<EndianVec<LittleEndian>> {
        // TODO: Look into how we can move on from `.borrow` as it is deprecated
        // Just copy the addresses for now
        let mut out_dwarf = write::Dwarf::from(&self.dwarf, &|address| {
            Some(write::Address::Constant(address))
        })
        .expect("cannot convert to writable dwarf");

        // Get all the unit headers from the `.debug_info` section
        let units = {
            let mut read_unit_headers = self.dwarf.units();
            // Output units
            let mut units = vec![];

            while let Some(u) = read_unit_headers.next().expect("") {
                let index = units.len();
                units.push((u, out_dwarf.units.id(index)));
            }
            units
        };
        for (unit_header, id) in units.iter() {
            let mut u = out_dwarf.units.get_mut(*id);
            let from_unit: Unit<EndianSlice<'_, LittleEndian>, usize> =
                self.dwarf.unit(*unit_header).expect("readable unit");
            // If it has a line program, do that transformation
            if let Some(line_program) = self.convert_unit_line_program(from_unit, instr_map.clone())
            {
                u.line_program = line_program;
            }
        }

        let mut sections = write::Sections::new(write::EndianVec::new(gimli::LittleEndian));
        out_dwarf.write(&mut sections).expect("write failed");
        sections
    }

    fn convert_unit_line_program(
        &mut self,
        from_unit: Unit<EndianSlice<LittleEndian>, usize>,
        instr_map: HashMap<usize, usize>,
    ) -> Option<write::LineProgram> {
        match from_unit.line_program {
            Some(ref from_program) => {
                let from_program = from_program.clone();
                let line_program = self
                    .convert_line_program(from_program, instr_map)
                    .expect("cannot convert line program");
                Some(line_program)
            }
            None => None,
        }
    }

    fn convert_line_string(
        &mut self,
        from_attr: AttributeValue<EndianSlice<LittleEndian>, usize>,
    ) -> write::ConvertResult<write::LineString> {
        Ok(match from_attr {
            read::AttributeValue::String(r) => write::LineString::String(r.to_slice()?.to_vec()),
            read::AttributeValue::DebugStrRef(offset) => {
                let r = self.dwarf.debug_str.get_str(offset)?;
                let id = self.strings.add(r.to_slice()?);
                write::LineString::StringRef(id)
            }
            read::AttributeValue::DebugLineStrRef(offset) => {
                let r = self.dwarf.debug_line_str.get_str(offset)?;
                let id = self.line_strings.add(r.to_slice()?);
                write::LineString::LineStringRef(id)
            }
            _ => return Err(write::ConvertError::UnsupportedLineStringForm),
        })
    }

    fn convert_line_program_header(
        &mut self,
        from_program: &read::IncompleteLineProgram<EndianSlice<LittleEndian>, usize>,
        files: &mut Vec<write::FileId>,
    ) -> write::ConvertResult<write::LineProgram> {
        let from_header = from_program.header();
        let encoding = from_header.encoding();

        let comp_dir = match from_header.directory(0) {
            Some(comp_dir) => self.convert_line_string(comp_dir)?,
            None => write::LineString::new(&[][..], encoding, &mut self.line_strings),
        };

        let (comp_name, comp_file_info) = match from_header.file(0) {
            Some(comp_file) => {
                if comp_file.directory_index() != 0 {
                    return Err(write::ConvertError::InvalidDirectoryIndex);
                }
                (
                    self.convert_line_string(comp_file.path_name())?,
                    Some(write::FileInfo {
                        timestamp: comp_file.timestamp(),
                        size: comp_file.size(),
                        md5: *comp_file.md5(),
                        // TODO: Update soure
                        source: None,
                    }),
                )
            }
            None => (
                write::LineString::new(&[][..], encoding, &mut self.line_strings),
                None,
            ),
        };

        if from_header.line_base() > 0 {
            return Err(write::ConvertError::InvalidLineBase);
        }
        let mut program = write::LineProgram::new(
            encoding,
            from_header.line_encoding(),
            comp_dir,
            comp_name,
            comp_file_info,
        );

        let mut dirs = Vec::new();
        let file_skip = if from_header.version() <= 4 {
            // The first directory is implicit.
            dirs.push(program.default_directory());
            // A file index of 0 is invalid for version <= 4, but putting
            // something there makes the indexing easier.
            0
        } else {
            // We don't add the first file to `files`, but still allow
            // it to be referenced from converted instructions.
            1
        };

        for from_dir in from_header.include_directories() {
            let from_dir = self.convert_line_string(from_dir.clone())?;
            dirs.push(program.add_directory(from_dir));
        }

        for from_file in from_header.file_names().iter().skip(file_skip) {
            let from_name = self.convert_line_string(from_file.path_name())?;
            let from_dir = from_file.directory_index();
            if from_dir >= dirs.len() as u64 {
                return Err(write::ConvertError::InvalidDirectoryIndex);
            }
            let from_dir = dirs[from_dir as usize];
            let from_info = Some(write::FileInfo {
                timestamp: from_file.timestamp(),
                size: from_file.size(),
                md5: *from_file.md5(),
                // TODO: Update source
                source: None,
            });
            files.push(program.add_file(from_name, from_dir, from_info));
        }

        Ok(program)
    }

    pub fn convert_line_program(
        &mut self,
        mut from_program: IncompleteLineProgram<EndianSlice<LittleEndian>, usize>,
        instr_map: HashMap<usize, usize>,
    ) -> write::ConvertResult<write::LineProgram> {
        // Create mappings in case the source has duplicate files or directories.
        let mut files = Vec::new();

        let mut program = self
            .convert_line_program_header(&from_program, &mut files)
            .expect("line program header cannot be converted");

        // We can't use the `from_program.rows()` because that wouldn't let
        // us preserve address relocations.
        let mut from_row = read::LineRow::new(from_program.header());
        let mut instructions = from_program.header().instructions();
        let mut address = None;
        while let Some(instruction) = instructions.next_instruction(from_program.header())? {
            match instruction {
                read::LineInstruction::SetAddress(val) => {
                    if program.in_sequence() {
                        return Err(ConvertError::UnsupportedLineInstruction);
                    }
                    if instr_map.contains_key(&(val as usize)) {
                        match instr_map.get(&(val as usize)) {
                            Some(val) => address = Some(write::Address::Constant(*val as u64)),
                            None => return Err(ConvertError::InvalidAddress),
                        }
                    }
                    from_row.execute(read::LineInstruction::SetAddress(0), &mut from_program)?;
                }
                read::LineInstruction::DefineFile(_) => {
                    return Err(ConvertError::UnsupportedLineInstruction);
                }
                _ => {
                    if from_row.execute(instruction, &mut from_program)? {
                        if !program.in_sequence() {
                            program.begin_sequence(address);
                            address = None;
                        }
                        if from_row.end_sequence() {
                            program.end_sequence(from_row.address());
                        } else {
                            program.row().address_offset = from_row.address();
                            program.row().op_index = from_row.op_index();
                            program.row().file = {
                                let file = from_row.file_index();
                                if file >= files.len() as u64 {
                                    return Err(ConvertError::InvalidFileIndex);
                                }
                                if file == 0 && program.version() <= 4 {
                                    return Err(ConvertError::InvalidFileIndex);
                                }
                                files[file as usize]
                            };
                            program.row().line = match from_row.line() {
                                Some(line) => line.get(),
                                None => 0,
                            };
                            program.row().column = match from_row.column() {
                                read::ColumnType::LeftEdge => 0,
                                read::ColumnType::Column(val) => val.get(),
                            };
                            program.row().discriminator = from_row.discriminator();
                            program.row().is_statement = from_row.is_stmt();
                            program.row().basic_block = from_row.basic_block();
                            program.row().prologue_end = from_row.prologue_end();
                            program.row().epilogue_begin = from_row.epilogue_begin();
                            program.row().isa = from_row.isa();
                            program.generate_row();
                        }
                        from_row.reset(from_program.header());
                    }
                }
            };
        }
        Ok(program)
    }
}

// NOTE: WALRUS IMPLEMENTATION FOR REFERENCE
// pub(crate) static DEAD_CODE: u64 = 0xFFFFFFFF;
//
// impl<R> ModuleDebugData<'_, R> where R: Reader<Offset = usize> {
//     pub fn convert_address(&mut self, instr_map: HashMap<usize, usize>, code_sec_offset: u64) {
//
//         let from_dwarf = self.dwarf
//             .borrow(|sections| EndianSlice::new(sections.as_ref(), LittleEndian));
//
//         let mut dwarf = write::Dwarf::from(&from_dwarf, &|address| {
//             if address == 0 || address == DEAD_CODE {
//                 Some(write::Address::Constant(address))
//             } else {
//                 if instr_map.contains_key(&(address as usize)) {
//                     Some(write::Address::Constant(*instr_map.get(&(address as usize)).unwrap() as u64 + code_sec_offset))
//                 } else {
//                     Some(write::Address::Constant(address))
//                 }
//             }
//         })
//             .expect("cannot convert to writable dwarf");
//
//         let units = {
//             let mut from_unit_headers = from_dwarf.units();
//             let mut units = Vec::new();
//
//             while let Some(from_unit) = from_unit_headers.next().expect("") {
//                 let index = units.len();
//                 units.push((from_unit, dwarf.units.id(index)));
//             }
//
//             units
//         };
//
//         for (from_id, id) in units {
//             let from_unit: Unit<EndianSlice<'_, LittleEndian>, usize> =
//                 from_dwarf.unit(from_id).expect("readable unit");
//             let unit = dwarf.units.get_mut(id);
//
//             // perform high pc transformation of DWARF .debug_info
//             // {
//             //     let mut from_entries = from_unit.entries();
//             //     let mut entries = DebuggingInformationCursor::new(unit);
//             //
//             //     convert_context.convert_high_pc(&mut from_entries, &mut entries);
//             // }
//
//             // perform line program transformation
//             if let Some(program) = self.convert_unit_line_program(from_unit, &instr_map, code_sec_offset, &dwarf) {
//                 unit.line_program = program;
//             }
//         }
//     }
//
//     pub(crate) fn convert_unit_line_program(
//         &mut self,
//         from_unit: read::Unit<R>,
//         instr_map: &HashMap<usize, usize>, code_sec_offset: u64, dwarf: &gimli::write::Dwarf,
//     ) -> Option<write::LineProgram> {
//         match from_unit.line_program {
//             Some(ref from_program) => {
//                 let from_program = from_program.clone();
//                 let line_program = self
//                     .convert_line_program(from_program, instr_map, code_sec_offset, dwarf)
//                     .expect("cannot convert line program");
//                 Some(line_program)
//             }
//             None => None,
//         }
//     }
//
//     fn convert_line_program_header(
//         &mut self,
//         from_program: &read::IncompleteLineProgram<R>,
//         files: &mut Vec<write::FileId>, dwarf: &gimli::write::Dwarf,
//     ) -> write::ConvertResult<write::LineProgram> {
//         let from_header = from_program.header();
//         let encoding = from_header.encoding();
//
//         let comp_dir = match from_header.directory(0) {
//             Some(comp_dir) => self.convert_line_string(comp_dir)?,
//             None => write::LineString::new(&[][..], encoding, dwarf.line_strings),
//         };
//
//         let x = (
//             write::LineString::new(&[][..], encoding, *dwarf.line_strings),
//             None,
//         );
//         let (comp_name, comp_file_info) = match from_header.file(0) {
//             Some(comp_file) => {
//                 if comp_file.directory_index() != 0 {
//                     return Err(write::ConvertError::InvalidDirectoryIndex);
//                 }
//                 (
//                     self.convert_line_string(comp_file.path_name())?,
//                     Some(write::FileInfo {
//                         timestamp: comp_file.timestamp(),
//                         size: comp_file.size(),
//                         md5: *comp_file.md5(),
//                         source: comp_file.source(),
//                     }),
//                 )
//             }
//             None => x,
//         };
//
//         if from_header.line_base() > 0 {
//             return Err(write::ConvertError::InvalidLineBase);
//         }
//         let mut program = write::LineProgram::new(
//             encoding,
//             from_header.line_encoding(),
//             comp_dir,
//             comp_name,
//             comp_file_info,
//         );
//
//         let mut dirs = Vec::new();
//         let file_skip = if from_header.version() <= 4 {
//             // The first directory is implicit.
//             dirs.push(program.default_directory());
//             // A file index of 0 is invalid for version <= 4, but putting
//             // something there makes the indexing easier.
//             0
//         } else {
//             // We don't add the first file to `files`, but still allow
//             // it to be referenced from converted instructions.
//             1
//         };
//
//         for from_dir in from_header.include_directories() {
//             let from_dir = self.convert_line_string(from_dir.clone())?;
//             dirs.push(program.add_directory(from_dir));
//         }
//
//         for from_file in from_header.file_names().iter().skip(file_skip) {
//             let from_name = self.convert_line_string(from_file.path_name())?;
//             let from_dir = from_file.directory_index();
//             if from_dir >= dirs.len() as u64 {
//                 return Err(write::ConvertError::InvalidDirectoryIndex);
//             }
//             let from_dir = dirs[from_dir as usize];
//             let from_info = Some(write::FileInfo {
//                 timestamp: from_file.timestamp(),
//                 size: from_file.size(),
//                 md5: *from_file.md5(),
//                 source: from_file.source()
//             });
//             files.push(program.add_file(from_name, from_dir, from_info));
//         }
//
//         Ok(program)
//     }
//
//
//
//     pub fn convert_line_program<R: Reader<Offset=usize>>(
//         &mut self,
//         mut from_program: read::IncompleteLineProgram<R>,
//         instr_map: &HashMap<usize, usize>,
//         code_sec_offset: u64,
//         dwarf: &Dwarf<R>,
//     ) -> ConvertResult<LineProgram> {
//         // Create mappings in case the source has duplicate files or directories.
//         let mut files = Vec::new();
//         let mut program = self
//             .convert_line_program_header(&from_program, &mut files, dwarf)
//             .expect("line program header cannot be converted");
//
//         // We can't use the `from_program.rows()` because that wouldn't let
//         // us preserve address relocations.
//         let mut from_row = read::LineRow::new(from_program.header());
//         let mut instructions = from_program.header().instructions();
//         let mut address = None;
//         let mut from_base_address = 0;
//
//         while let Some(instruction) = instructions.next_instruction(from_program.header())? {
//             match instruction {
//                 read::LineInstruction::SetAddress(val) => {
//                     if program.in_sequence() {
//                         return Err(ConvertError::UnsupportedLineInstruction);
//                     }
//                     from_base_address = val;
//
//                     from_row.execute(read::LineInstruction::SetAddress(0), &mut from_program);
//                 }
//                 read::LineInstruction::DefineFile(_) => {
//                     return Err(ConvertError::UnsupportedLineInstruction);
//                 }
//                 _ => {
//                     if from_row.execute(instruction, &mut from_program)? {
//                         if !program.in_sequence() {
//                             program.begin_sequence(address);
//                             address = None;
//                         }
//                         if from_row.end_sequence() {
//                             program.end_sequence(from_row.address());
//                         } else {
//                             program.row().address_offset = from_row.address();
//                             program.row().op_index = from_row.op_index();
//                             program.row().file = {
//                                 let file = from_row.file_index();
//                                 if file >= files.len() as u64 {
//                                     return Err(ConvertError::InvalidFileIndex);
//                                 }
//                                 if file == 0 && program.version() <= 4 {
//                                     return Err(ConvertError::InvalidFileIndex);
//                                 }
//                                 files[file as usize]
//                             };
//                             program.row().line = match from_row.line() {
//                                 Some(line) => line.get() + *instr_map.get(&(line.get() as usize)).unwrap() as u64 + code_sec_offset as u64,
//                                 None => 0,
//                             };
//                             program.row().column = match from_row.column() {
//                                 read::ColumnType::LeftEdge => 0,
//                                 read::ColumnType::Column(val) => val.get(),
//                             };
//                             program.row().discriminator = from_row.discriminator();
//                             program.row().is_statement = from_row.is_stmt();
//                             program.row().basic_block = from_row.basic_block();
//                             program.row().prologue_end = from_row.prologue_end();
//                             program.row().epilogue_begin = from_row.epilogue_begin();
//                             program.row().isa = from_row.isa();
//                             program.generate_row();
//                         }
//                         from_row.reset(from_program.header());
//                     }
//                 }
//             };
//         }
//         Ok((program, files))
//
//     }
// }
