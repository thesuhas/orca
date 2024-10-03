use gimli::read::Dwarf;
use gimli::write::{ConvertError, ConvertResult, LineProgram};
use gimli::{read, write, EndianSlice, LittleEndian, Reader, Unit};
use std::collections::HashMap;

/// The DWARF debug section in input WebAssembly binary.
#[derive(Debug, Default)]
pub struct ModuleDebugData<'a> {
    /// DWARF debug data
    pub dwarf: Dwarf<EndianSlice<'a, LittleEndian>>,
}

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
