//! Intermediate Representation of a wasm module.

use crate::error::Error;
use crate::ir::function::FunctionModifier;
use crate::ir::id::{DataSegmentID, FunctionID, GlobalID, ImportsID, LocalID, TypeID};
use crate::ir::types::Instrument::{Instrumented, NotInstrumented};
use crate::ir::types::{
    Body, DataSegment, DataSegmentKind, ElementItems, ElementKind, FuncType, Global, Instrument,
};
use wasm_encoder::reencode::Reencode;
use wasmparser::{Export, MemoryType, Operator, Parser, Payload, TableType};

use super::types::DataType;
use crate::ir::section::ModuleSection;
use wasm_encoder::NameMap;

#[derive(Clone, Debug)]
/// Intermediate Representation of a wasm module.
/// See https://webassembly.github.io/spec/core/binary/modules.html as a reference
/// for different sections
pub struct Module<'a> {
    /// Types
    pub types: Vec<FuncType>,
    /// Imports
    pub imports: Vec<crate::ir::types::Import<'a>>,
    /// Mapping from function index to type index.
    /// Note that |functions| == |code_sections| == num_functions
    pub functions: Vec<TypeID>,
    /// Each table has a type and optional initialization expression.
    pub tables: Vec<(TableType, Option<wasmparser::ConstExpr<'a>>)>,
    /// Memories
    pub memories: Vec<MemoryType>,
    /// Globals
    pub globals: Vec<Global>,
    /// Data Sections
    pub data: Vec<DataSegment>,
    pub data_count_section_exists: bool,
    /// Exports
    pub exports: Vec<Export<'a>>,
    /// Index of the start function.
    pub start: Option<FunctionID>,
    /// Elements
    pub elements: Vec<(ElementKind<'a>, ElementItems<'a>)>,
    /// Function Bodies
    pub code_sections: Vec<Body<'a>>,
    /// Custom Sections
    pub custom_sections: Vec<(&'a str, &'a [u8])>,
    /// Number of local functions (not counting imported functions)
    pub num_functions: usize,
    pub num_imported_functions: usize,
    /// name of module
    pub module_name: Option<String>,
    /// Sections of the Module. Represented as (#num of occurrences of a section, type of section)
    pub sections: Vec<(u32, ModuleSection)>,
    num_sections: usize,

    // just a placeholder for roundtrip
    pub(crate) local_names: wasm_encoder::IndirectNameMap,
}

impl<'a> Module<'a> {
    /// Parses a `Module` from a wasm binary.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use orca::Module;
    ///
    /// let file = "path_to_file";
    /// let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    /// let module = Module::parse(&buff, false).unwrap();
    /// ```
    pub fn parse(wasm: &'a [u8], enable_multi_memory: bool) -> Result<Self, Error> {
        let parser = Parser::new(0);
        Module::parse_internal(wasm, enable_multi_memory, parser)
    }

    fn add_to_sections(
        sections: &mut Vec<(u32, ModuleSection)>,
        section: ModuleSection,
        num_sections: &mut usize,
        sections_added: u32,
    ) {
        if *num_sections > 0 && sections[*num_sections - 1].1 == section {
            sections[*num_sections - 1].0 += sections_added;
        } else {
            sections.push((sections_added, section));
            *num_sections += 1;
        }
    }

    pub(crate) fn parse_internal(
        wasm: &'a [u8],
        enable_multi_memory: bool,
        parser: Parser,
    ) -> Result<Self, Error> {
        let wasm_features = wasmparser::WasmFeatures::default();
        let mut imports: Vec<crate::ir::types::Import> = vec![];
        let mut types = vec![];
        let mut data = vec![];
        let mut tables = vec![];
        let mut memories = vec![];
        let mut functions = vec![];
        let mut elements = vec![];
        let mut code_section_count = 0;
        let mut code_sections = vec![];
        let mut globals = vec![];
        let mut exports = vec![];
        let mut start = None;
        let mut data_section_count = None;
        let mut custom_sections = vec![];
        let mut num_imported_functions = 0;
        let mut num_sections: usize = 0;
        let mut sections = vec![];

        let mut module_name: Option<String> = None;
        // for the other names, we directly encode it without passing them into the IR
        let mut local_names = wasm_encoder::IndirectNameMap::new();

        for payload in parser.parse_all(wasm) {
            let payload = payload?;
            match payload {
                Payload::ImportSection(import_section_reader) => {
                    let mut temp = vec![];
                    // count number of imported functions
                    for import in import_section_reader.into_iter() {
                        let imp = crate::ir::types::Import::from(import?);
                        if imp.is_function() {
                            num_imported_functions += 1;
                        }
                        temp.push(imp);
                    }
                    let l = temp.len();
                    imports.append(&mut temp);
                    Self::add_to_sections(
                        &mut sections,
                        ModuleSection::Import,
                        &mut num_sections,
                        l as u32,
                    );
                }
                Payload::TypeSection(type_section_reader) => {
                    let mut temp = vec![];
                    for ty in type_section_reader.into_iter_err_on_gc_types() {
                        let fun_ty = ty?;
                        let params = fun_ty
                            .params()
                            .iter()
                            .map(|x| DataType::from(*x))
                            .collect::<Vec<_>>()
                            .into_boxed_slice();
                        let results = fun_ty
                            .results()
                            .iter()
                            .map(|x| DataType::from(*x))
                            .collect::<Vec<_>>()
                            .into_boxed_slice();

                        temp.push(FuncType::new(params, results));
                    }
                    let l = temp.len();
                    types.append(&mut temp);
                    Self::add_to_sections(
                        &mut sections,
                        ModuleSection::FuncType,
                        &mut num_sections,
                        l as u32,
                    );
                }
                Payload::DataSection(data_section_reader) => {
                    let temp: &mut Vec<DataSegment> = &mut data_section_reader
                        .into_iter()
                        .map(|sec| {
                            sec.map_err(Error::from)
                                .and_then(DataSegment::from_wasmparser)
                        })
                        .collect::<Result<_, _>>()?;
                    let l = temp.len();
                    data.append(temp);
                    Self::add_to_sections(
                        &mut sections,
                        ModuleSection::DataSegment,
                        &mut num_sections,
                        l as u32,
                    );
                }
                Payload::TableSection(table_section_reader) => {
                    let temp: &mut Vec<(TableType, Option<wasmparser::ConstExpr<'a>>)> =
                        &mut table_section_reader
                            .into_iter()
                            .map(|t| {
                                t.map_err(Error::from).map(|t| match t.init {
                                    wasmparser::TableInit::RefNull => (t.ty, None),
                                    wasmparser::TableInit::Expr(e) => (t.ty, Some(e)),
                                })
                            })
                            .collect::<Result<_, _>>()?;
                    let l = temp.len();
                    tables.append(temp);
                    Self::add_to_sections(
                        &mut sections,
                        ModuleSection::Table,
                        &mut num_sections,
                        l as u32,
                    );
                }
                Payload::MemorySection(memory_section_reader) => {
                    let temp: &mut Vec<MemoryType> = &mut memory_section_reader
                        .into_iter()
                        .collect::<Result<_, _>>()?;
                    let l = temp.len();
                    memories.append(temp);
                    Self::add_to_sections(
                        &mut sections,
                        ModuleSection::Memory,
                        &mut num_sections,
                        l as u32,
                    );
                }
                Payload::FunctionSection(function_section_reader) => {
                    let temp: Vec<u32> = function_section_reader
                        .into_iter()
                        .collect::<Result<_, _>>()?;
                    let l = temp.len();
                    functions.extend(temp.iter().map(|id| *id as TypeID));
                    Self::add_to_sections(
                        &mut sections,
                        ModuleSection::Function,
                        &mut num_sections,
                        l as u32,
                    );
                }
                Payload::GlobalSection(global_section_reader) => {
                    let temp: &mut Vec<Global> = &mut global_section_reader
                        .into_iter()
                        .map(|g| Global::from_wasmparser(g?))
                        .collect::<Result<_, _>>()?;
                    let l = temp.len();
                    globals.append(temp);
                    Self::add_to_sections(
                        &mut sections,
                        ModuleSection::Global,
                        &mut num_sections,
                        l as u32,
                    );
                }
                Payload::ExportSection(export_section_reader) => {
                    let temp: &mut Vec<Export> = &mut export_section_reader
                        .into_iter()
                        .collect::<Result<_, _>>()?;
                    let l = temp.len();
                    exports.append(temp);
                    Self::add_to_sections(
                        &mut sections,
                        ModuleSection::Export,
                        &mut num_sections,
                        l as u32,
                    );
                }
                Payload::StartSection { func, range: _ } => {
                    if start.is_some() {
                        return Err(Error::MultipleStartSections);
                    }
                    start = Some(func as FunctionID);
                    Self::add_to_sections(
                        &mut sections,
                        ModuleSection::Start,
                        &mut num_sections,
                        1,
                    );
                }
                Payload::ElementSection(element_section_reader) => {
                    let mut temp = vec![];
                    for element in element_section_reader.into_iter() {
                        let element = element?;
                        let items = ElementItems::from_wasmparser(element.items.clone())?;
                        temp.push((ElementKind::from_wasmparser(element.kind)?, items));
                    }
                    let l = temp.len();
                    elements.append(&mut temp);
                    Self::add_to_sections(
                        &mut sections,
                        ModuleSection::Elements,
                        &mut num_sections,
                        l as u32,
                    );
                }
                Payload::DataCountSection { count, range: _ } => {
                    data_section_count = Some(count);
                }
                Payload::CodeSectionStart {
                    count,
                    range: _,
                    size: _,
                } => {
                    code_section_count = count as usize;
                }
                Payload::CodeSectionEntry(body) => {
                    let locals_reader = body.get_locals_reader()?;
                    let locals = locals_reader.into_iter().collect::<Result<Vec<_>, _>>()?;
                    let locals: Vec<(u32, DataType)> = locals
                        .iter()
                        .map(|(idx, valtype)| (*idx, DataType::from(*valtype)))
                        .collect();
                    // TODO: can I just iter locals once?
                    let num_locals = locals.iter().fold(0, |acc, x| acc + x.0) as usize;
                    let instructions = body
                        .get_operators_reader()?
                        .into_iter()
                        .collect::<Result<Vec<_>, _>>()?;
                    if let Some(last) = instructions.last() {
                        if let Operator::End = last {
                        } else {
                            return Err(Error::MissingFunctionEnd {
                                func_range: body.range(),
                            });
                        }
                    }
                    if !enable_multi_memory
                        && instructions.iter().any(|i| match i {
                            Operator::MemoryGrow { mem, .. } | Operator::MemorySize { mem, .. } => {
                                *mem != 0x00
                            }
                            _ => false,
                        })
                    {
                        return Err(Error::InvalidMemoryReservedByte {
                            func_range: body.range(),
                        });
                    }
                    let instructions_bool: Vec<_> = instructions
                        .into_iter()
                        .map(|op| (op, Instrument::NotInstrumented))
                        .collect();
                    code_sections.push(Body {
                        locals,
                        num_locals,
                        instructions: instructions_bool.clone(),
                        num_instructions: instructions_bool.len(),
                        name: None,
                    });
                    Self::add_to_sections(
                        &mut sections,
                        ModuleSection::CodeSection,
                        &mut num_sections,
                        1,
                    );
                }
                Payload::CustomSection(custom_section_reader) => {
                    match custom_section_reader.name() {
                        "name" => {
                            let name_section_reader =
                                wasmparser::NameSectionReader::new(wasmparser::BinaryReader::new(
                                    custom_section_reader.data(),
                                    custom_section_reader.data_offset(),
                                    wasm_features,
                                ));
                            for subsection in name_section_reader {
                                #[allow(clippy::single_match)]
                                match subsection? {
                                    wasmparser::Name::Function(names) => {
                                        for name in names {
                                            let naming = name?;
                                            let abs_idx = naming.index;
                                            if abs_idx < num_imported_functions as u32 {
                                                let mut import_func_count = 0;
                                                // TODO: this is very expensive, can we optimize this?
                                                for import in imports.iter_mut() {
                                                    if import.is_function() {
                                                        if import_func_count == abs_idx {
                                                            import.import_name =
                                                                Some(naming.name.to_string());
                                                            break;
                                                        }
                                                        import_func_count += 1;
                                                    }
                                                }
                                            } else {
                                                let rel_idx =
                                                    abs_idx - num_imported_functions as u32;
                                                // assert!(0 < rel_idx && rel_idx < code_sections.len() as u32);
                                                code_sections[rel_idx as usize].name =
                                                    Some(naming.name.to_string());
                                            }
                                        }
                                    }
                                    wasmparser::Name::Module { name, .. } => {
                                        module_name = Some(name.to_string());
                                    }
                                    wasmparser::Name::Local(names) => {
                                        local_names = indirect_namemap_parser2encoder(names);
                                    }
                                    wasmparser::Name::Unknown { .. } => {}
                                    _ => {
                                        // we do nothing for the extended name section proposal
                                        // https://github.com/WebAssembly/extended-name-section/blob/main/proposals/extended-name-section/Overview.md
                                        // we could preserve them like what we did for locals
                                    }
                                }
                            }
                        }
                        _ => {
                            custom_sections
                                .push((custom_section_reader.name(), custom_section_reader.data()));
                        }
                    }
                    Self::add_to_sections(
                        &mut sections,
                        ModuleSection::CustomSection,
                        &mut num_sections,
                        1,
                    );
                }
                Payload::Version {
                    num,
                    encoding: _,
                    range: _,
                } => {
                    if num != 1 {
                        return Err(Error::UnknownVersion(num as u32));
                    }
                }
                Payload::UnknownSection {
                    id,
                    contents: _,
                    range: _,
                } => return Err(Error::UnknownSection { section_id: id }),
                Payload::TagSection(_)
                | Payload::ModuleSection {
                    parser: _,
                    unchecked_range: _,
                }
                | Payload::InstanceSection(_)
                | Payload::CoreTypeSection(_)
                | Payload::ComponentSection {
                    parser: _,
                    unchecked_range: _,
                }
                | Payload::ComponentInstanceSection(_)
                | Payload::ComponentAliasSection(_)
                | Payload::ComponentTypeSection(_)
                | Payload::ComponentCanonicalSection(_)
                | Payload::ComponentStartSection { start: _, range: _ }
                | Payload::ComponentImportSection(_)
                | Payload::ComponentExportSection(_)
                | Payload::End(_) => {}
            }
        }
        if code_section_count != code_sections.len() || code_section_count != functions.len() {
            return Err(Error::IncorrectCodeCounts {
                function_section_count: functions.len(),
                code_section_declared_count: code_section_count,
                code_section_actual_count: code_sections.len(),
            });
        }
        if let Some(data_count) = data_section_count {
            if data_count as usize != data.len() {
                return Err(Error::IncorrectDataCount {
                    declared_count: data_count as usize,
                    actual_count: data.len(),
                });
            }
        }
        Ok(Module {
            types,
            imports,
            functions,
            tables,
            memories,
            globals,
            exports,
            start,
            elements,
            data_count_section_exists: data_section_count.is_some(),
            code_sections: code_sections.clone(),
            data,
            custom_sections,
            num_functions: code_sections.len(),
            num_imported_functions,
            module_name,
            sections,
            num_sections,
            local_names,
        })
    }

    /// Emit the module into a wasm binary file.
    pub fn emit_wasm(&self, file_name: &str) -> Result<(), std::io::Error> {
        let module = self.encode_internal();
        let wasm = module.finish();
        std::fs::write(file_name, wasm)?;
        Ok(())
    }

    /// Encode the module into a wasm binary.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use orca::Module;
    ///
    /// let file = "path_to_file";
    /// let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    /// let module = Module::parse(&buff, false).unwrap();
    /// let result = module.encode();
    /// ```
    pub fn encode(&self) -> Vec<u8> {
        self.encode_internal().finish()
    }

    /// Encodes an Orca Module to a wasm_encoder Module
    pub(crate) fn encode_internal(&self) -> wasm_encoder::Module {
        let mut module = wasm_encoder::Module::new();
        let mut reencode = wasm_encoder::reencode::RoundtripReencoder;

        if !self.types.is_empty() {
            let mut types = wasm_encoder::TypeSection::new();

            for ty in self.types.iter() {
                let params = ty
                    .params
                    .iter()
                    .map(wasm_encoder::ValType::from)
                    .collect::<Vec<_>>();
                let results = ty
                    .results
                    .iter()
                    .map(wasm_encoder::ValType::from)
                    .collect::<Vec<_>>();

                types.function(params, results);
            }
            module.section(&types);
        }

        // initialize function name seciton
        let mut function_names = NameMap::new();
        if !self.imports.is_empty() {
            let mut imports = wasm_encoder::ImportSection::new();
            let mut import_func_idx = 0;
            for import in self.imports.iter() {
                if import.is_function() {
                    if let Some(import_name) = &import.import_name {
                        function_names.append(import_func_idx as u32, import_name);
                    }
                    import_func_idx += 1;
                }
                imports.import(
                    import.module,
                    import.name,
                    reencode.entity_type(import.ty).unwrap(),
                );
            }
            module.section(&imports);
        }

        if !self.functions.is_empty() {
            let mut functions = wasm_encoder::FunctionSection::new();
            for type_index in self.functions.iter() {
                functions.function(*type_index);
            }
            module.section(&functions);
        }

        if !self.tables.is_empty() {
            let mut tables = wasm_encoder::TableSection::new();
            for (table_ty, init) in self.tables.iter() {
                let table_ty = wasm_encoder::TableType {
                    element_type: wasm_encoder::RefType {
                        nullable: table_ty.element_type.is_nullable(),
                        heap_type: reencode
                            .heap_type(table_ty.element_type.heap_type())
                            .unwrap(),
                    },
                    table64: table_ty.table64,
                    minimum: table_ty.initial, // TODO - Check if this maps
                    maximum: table_ty.maximum,
                    shared: table_ty.shared,
                };
                match init {
                    None => tables.table(table_ty),
                    Some(const_expr) => tables.table_with_init(
                        table_ty,
                        &reencode
                            .const_expr((*const_expr).clone())
                            .expect("Error in Converting Const Expr"),
                    ),
                };
            }
            module.section(&tables);
        }

        if !self.memories.is_empty() {
            let mut memories = wasm_encoder::MemorySection::new();
            for memory in self.memories.iter() {
                memories.memory(wasm_encoder::MemoryType::from(*memory));
            }
            module.section(&memories);
        }

        if !self.globals.is_empty() {
            let mut globals = wasm_encoder::GlobalSection::new();
            for global in self.globals.iter() {
                globals.global(
                    wasm_encoder::GlobalType {
                        val_type: reencode.val_type(global.ty.content_type).unwrap(),
                        mutable: global.ty.mutable,
                        shared: global.ty.shared,
                    },
                    &(*global).clone().init_expr.to_wasmencoder_type(),
                );
            }
            module.section(&globals);
        }

        if !self.exports.is_empty() {
            let mut exports = wasm_encoder::ExportSection::new();
            for export in self.exports.iter() {
                exports.export(
                    export.name,
                    wasm_encoder::ExportKind::from(export.kind),
                    export.index,
                );
            }
            module.section(&exports);
        }

        if let Some(function_index) = self.start {
            module.section(&wasm_encoder::StartSection { function_index });
        }

        if !self.elements.is_empty() {
            let mut elements = wasm_encoder::ElementSection::new();
            let mut temp_const_exprs = vec![];
            for (kind, items) in self.elements.iter() {
                temp_const_exprs.clear();
                let element_items = match &items {
                    ElementItems::Functions(funcs) => wasm_encoder::Elements::Functions(funcs),
                    ElementItems::ConstExprs { ty, exprs } => {
                        temp_const_exprs.reserve(exprs.len());
                        for e in exprs.iter() {
                            temp_const_exprs.push(
                                reencode
                                    .const_expr((*e).clone())
                                    .expect("Unable to convert element constant expr"),
                            );
                        }
                        wasm_encoder::Elements::Expressions(
                            wasm_encoder::RefType {
                                nullable: ty.is_nullable(),
                                heap_type: reencode.heap_type(ty.heap_type()).unwrap(),
                            },
                            &temp_const_exprs,
                        )
                    }
                };

                match kind {
                    ElementKind::Passive => {
                        elements.passive(element_items);
                    }
                    ElementKind::Active {
                        table_index,
                        offset_expr,
                    } => {
                        elements.active(
                            *table_index,
                            &reencode
                                .const_expr((*offset_expr).clone())
                                .expect("Unable to convert offset expr"),
                            element_items,
                        );
                    }
                    ElementKind::Declared => {
                        elements.declared(element_items);
                    }
                }
            }
            module.section(&elements);
        }

        if self.data_count_section_exists {
            let data_count = wasm_encoder::DataCountSection {
                count: self.data.len() as u32,
            };
            module.section(&data_count);
        }

        if !self.code_sections.is_empty() {
            let mut code = wasm_encoder::CodeSection::new();
            for (
                rel_func_idx,
                Body {
                    locals,
                    num_locals: _,
                    instructions,
                    num_instructions: _,
                    name,
                },
            ) in self.code_sections.iter().enumerate()
            {
                let mut converted_locals = Vec::with_capacity(locals.len());
                for (c, ty) in locals {
                    converted_locals.push((*c, wasm_encoder::ValType::from(ty)));
                }
                let mut function = wasm_encoder::Function::new(converted_locals);
                for (op, instrument) in instructions {
                    match instrument {
                        NotInstrumented => {
                            function.instruction(
                                &reencode
                                    .instruction((*op).clone())
                                    .expect("Unable to convert Instruction"),
                            );
                        }
                        Instrumented {
                            before,
                            after,
                            alternate,
                            current: _current,
                        } => {
                            // First encode before instructions
                            for instr in before {
                                function.instruction(
                                    &reencode
                                        .instruction(instr.clone())
                                        .expect("Unable to convert Instruction"),
                                );
                            }

                            // If there are any alternate, encode the alternate
                            if !alternate.is_empty() {
                                for instr in alternate {
                                    function.instruction(
                                        &reencode
                                            .instruction(instr.clone())
                                            .expect("Unable to convert Instruction"),
                                    );
                                }
                            } else {
                                function.instruction(
                                    &reencode
                                        .instruction((*op).clone())
                                        .expect("Unable to convert Instruction"),
                                );
                            }
                            // Now encode the after instructions
                            for instr in after {
                                function.instruction(
                                    &reencode
                                        .instruction(instr.clone())
                                        .expect("Unable to convert Instruction"),
                                );
                            }
                        }
                    }
                }
                if let Some(name) = name {
                    function_names.append(
                        self.num_imported_functions as u32 + rel_func_idx as u32,
                        name,
                    );
                }
                code.function(&function);
            }
            module.section(&code);
        }

        if !self.data.is_empty() {
            let mut data = wasm_encoder::DataSection::new();
            for segment in self.data.iter() {
                let segment_data = segment.data.iter().copied();
                match (*segment).clone().kind {
                    DataSegmentKind::Passive => data.passive(segment_data),
                    DataSegmentKind::Active {
                        memory_index,
                        offset_expr,
                    } => data.active(
                        memory_index,
                        &offset_expr.to_wasmencoder_type(),
                        segment_data,
                    ),
                };
            }
            module.section(&data);
        }

        // the name section is not stored in self.custom_sections anymore
        let mut names = wasm_encoder::NameSection::new();

        // we only encode the three name subsection in wasm spec
        if let Some(module_name) = &self.module_name {
            names.module(module_name);
        }
        names.functions(&function_names);
        names.locals(&self.local_names);

        module.section(&names);

        // encode the rest of custom sections
        for (name, data) in self.custom_sections.iter() {
            module.section(&wasm_encoder::CustomSection {
                name: std::borrow::Cow::Borrowed(name),
                data: std::borrow::Cow::Borrowed(data),
            });
        }

        module
    }

    /// Add a new type to the module, returns the index of the new type.
    pub fn add_type(&mut self, param: &[DataType], ret: &[DataType]) -> TypeID {
        let index = self.types.len();
        let ty = FuncType::new(
            param.to_vec().into_boxed_slice(),
            ret.to_vec().into_boxed_slice(),
        );
        self.types.push(ty);
        index as TypeID
    }

    /// Get type from index of the type section
    pub fn get_type(&self, index: TypeID) -> Option<&FuncType> {
        self.types.get(index as usize)
    }

    /// Add a new Global to the module. Returns the index of the new Global.
    pub fn add_global(&mut self, global: Global) -> GlobalID {
        let index = self.globals.len();
        self.globals.push(global);
        index as GlobalID
    }

    pub(crate) fn add_local(&mut self, func_idx: usize, ty: DataType) -> LocalID {
        // get type
        let func_ty = self.get_type(self.functions[func_idx]).unwrap();
        let num_params = func_ty.params.len();

        let func_body = &mut self.code_sections[func_idx];
        let num_locals = func_body.num_locals;
        let index = num_params + num_locals;

        let len = func_body.locals.len();
        func_body.num_locals += 1;
        if len > 0 {
            let last = len - 1;
            if func_body.locals[last].1 == ty {
                func_body.locals[last].0 += 1;
            } else {
                func_body.locals.push((1, ty));
            }
        } else {
            // If no locals, just append
            func_body.locals.push((1, ty));
        }

        index as LocalID
    }

    /// Add a new Data Segment to the module.
    /// Returns the index of the new Data Segment in the Data Section.
    pub fn add_data(&mut self, data: DataSegment) -> DataSegmentID {
        let index = self.data.len();
        self.data.push(data);
        index as DataSegmentID
    }

    /// Add a new function to the module. Returns the index of the imported function
    /// Note: this as no effect on the code or function section
    // TODO: In walrus, add_import_func after adding a function has no effect
    pub fn add_import_func(&mut self, module: &'a str, name: &'a str, ty_id: u32) -> ImportsID {
        let index = self.imports.len();
        let import = crate::ir::types::Import {
            module,
            name,
            ty: wasmparser::TypeRef::Func(ty_id),
            import_name: None,
        };
        self.imports.push(import);

        index as ImportsID
    }

    /// Count number of imported function
    pub fn num_import_func(&self) -> u32 {
        self.num_imported_functions as u32
    }

    /// Set a function name to a function using it's relative index
    pub fn set_fn_name(&mut self, func_idx: u32, name: &'a str) {
        let body = &mut self.code_sections[func_idx as usize];
        body.name = Some(name.to_owned());
    }

    /// Add an Export to a `Module`
    pub fn add_export_func(&mut self, name: &'a str, func_idx: u32) {
        let export = Export {
            name,
            kind: wasmparser::ExternalKind::Func,
            index: func_idx,
        };
        self.exports.push(export);
    }

    /// Get a function modifier from a function index
    pub fn get_fn<'b>(&'b mut self, func_id: FunctionID) -> Option<FunctionModifier<'b, 'a>> {
        // grab type and section and code section
        // let ty = self.functions.get(func_idx as usize)?;
        let body = self.code_sections.get_mut(func_id as usize)?;
        Some(FunctionModifier::init(body))
        // None
    }

    /// Get Local Function ID by name
    // Note: returned absolute id here
    pub fn get_fid_by_name(&self, name: &str) -> Option<u32> {
        for (idx, body) in self.code_sections.iter().enumerate() {
            if let Some(n) = &body.name {
                if n == name {
                    return Some(idx as u32 + self.num_imported_functions as u32);
                }
            }
        }
        None
    }

    /// Get a Function name from its Function ID
    pub fn get_fname(&self, id: FunctionID) -> Option<String> {
        if (id as usize) < self.code_sections.len() {
            self.code_sections[id as usize].name.clone()
        } else {
            None
        }
    }

    /// Create an empty Module
    pub fn new() -> Self {
        Module {
            types: vec![],
            imports: vec![],
            functions: vec![],
            tables: vec![],
            memories: vec![],
            globals: vec![],
            exports: vec![],
            start: None,
            elements: vec![],
            data_count_section_exists: false,
            code_sections: vec![],
            data: vec![],
            custom_sections: vec![],
            num_functions: 0,
            num_imported_functions: 0,
            module_name: None,
            sections: vec![],
            num_sections: 0,
            local_names: wasm_encoder::IndirectNameMap::new(),
        }
    }
}

impl<'a> Default for Module<'a> {
    fn default() -> Self {
        Self::new()
    }
}

fn indirect_namemap_parser2encoder(
    namemap: wasmparser::IndirectNameMap,
) -> wasm_encoder::IndirectNameMap {
    let mut names = wasm_encoder::IndirectNameMap::new();
    for name in namemap {
        let naming = name.unwrap();
        names.append(naming.index, &namemap_parser2encoder(naming.names));
    }
    names
}

fn namemap_parser2encoder(namemap: wasmparser::NameMap) -> wasm_encoder::NameMap {
    let mut names = wasm_encoder::NameMap::new();
    for name in namemap {
        let naming = name.unwrap();
        names.append(naming.index, naming.name);
    }
    names
}
