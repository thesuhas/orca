//! Intermediate Representation of a wasm module.

use crate::error::Error;
use crate::ir::id::{DataSegmentID, FunctionID, ImportsID, LocalID, MemoryID, TypeID};
use crate::ir::module::module_exports::{Export, ModuleExports};
use crate::ir::module::module_functions::{
    FuncKind, Function, Functions, ImportedFunction, LocalFunction,
};
use crate::ir::module::module_globals::{Global, ModuleGlobals};
use crate::ir::module::module_imports::{Import, ModuleImports};
use crate::ir::module::module_tables::ModuleTables;
use crate::ir::module::module_types::{FuncType, ModuleTypes};
use crate::ir::types::{
    Body, CustomSections, DataSegment, DataSegmentKind, ElementItems, ElementKind,
    InstrumentationFlag,
};
use wasm_encoder::reencode::Reencode;
use wasmparser::{MemoryType, Operator, Parser, Payload};

use super::types::DataType;
use crate::ir::wrappers::{indirect_namemap_parser2encoder, namemap_parser2encoder};

pub mod module_exports;
pub mod module_functions;
pub mod module_globals;
pub mod module_imports;
pub mod module_tables;
pub mod module_types;

#[derive(Clone, Debug)]
/// Intermediate Representation of a wasm module. See the [WASM Spec] for different sections.
///
/// [WASM Spec]: https://webassembly.github.io/spec/core/binary/modules.html
pub struct Module<'a> {
    /// Types
    pub types: ModuleTypes,
    /// Imports
    pub imports: ModuleImports<'a>,
    /// Mapping from function index to type index.
    /// Note that `|functions| == num_functions + num_imported_functions`
    pub functions: Functions<'a>,
    /// Each table has a type and optional initialization expression.
    pub tables: ModuleTables<'a>,
    /// Memories
    pub memories: Vec<MemoryType>,
    /// Globals
    pub globals: ModuleGlobals,
    /// Data Sections
    pub data: Vec<DataSegment>,
    pub data_count_section_exists: bool,
    /// Exports
    pub exports: ModuleExports,
    /// Index of the start function.
    pub start: Option<FunctionID>,
    /// Elements
    pub elements: Vec<(ElementKind<'a>, ElementItems<'a>)>,
    /// Custom Sections
    pub custom_sections: CustomSections<'a>,
    /// Number of local functions (not counting imported functions)
    pub num_functions: usize,
    /// Number of imported functions
    pub num_imported_functions: usize,
    /// name of module
    pub module_name: Option<String>,

    // just a placeholder for round-trip
    pub(crate) local_names: wasm_encoder::IndirectNameMap,
    pub(crate) label_names: wasm_encoder::IndirectNameMap,
    pub(crate) type_names: wasm_encoder::NameMap,
    pub(crate) table_names: wasm_encoder::NameMap,
    pub(crate) memory_names: wasm_encoder::NameMap,
    pub(crate) global_names: wasm_encoder::NameMap,
    pub(crate) elem_names: wasm_encoder::NameMap,
    pub(crate) data_names: wasm_encoder::NameMap,
    pub(crate) field_names: wasm_encoder::IndirectNameMap,
    pub(crate) tag_names: wasm_encoder::NameMap,
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

    pub(crate) fn parse_internal(
        wasm: &'a [u8],
        enable_multi_memory: bool,
        parser: Parser,
    ) -> Result<Self, Error> {
        let mut imports: Vec<Import> = vec![];
        let mut types: Vec<FuncType> = vec![];
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

        let mut module_name: Option<String> = None;
        // for the other names, we directly encode it without passing them into the IR
        let mut local_names = wasm_encoder::IndirectNameMap::new();
        let mut label_names = wasm_encoder::IndirectNameMap::new();
        let mut type_names = wasm_encoder::NameMap::new();
        let mut table_names = wasm_encoder::NameMap::new();
        let mut memory_names = wasm_encoder::NameMap::new();
        let mut global_names = wasm_encoder::NameMap::new();
        let mut elem_names = wasm_encoder::NameMap::new();
        let mut data_names = wasm_encoder::NameMap::new();
        let mut field_names = wasm_encoder::IndirectNameMap::new();
        let mut tag_names = wasm_encoder::NameMap::new();

        for payload in parser.parse_all(wasm) {
            let payload = payload?;
            match payload {
                Payload::ImportSection(import_section_reader) => {
                    let mut temp = vec![];
                    // count number of imported functions
                    for import in import_section_reader.into_iter() {
                        let imp = Import::from(import?);
                        if imp.is_function() {
                            num_imported_functions += 1;
                        }
                        temp.push(imp);
                    }
                    imports.append(&mut temp);
                }
                Payload::TypeSection(type_section_reader) => {
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

                        types.push(FuncType::new(params, results));
                    }
                }
                Payload::DataSection(data_section_reader) => {
                    data = data_section_reader
                        .into_iter()
                        .map(|sec| {
                            sec.map_err(Error::from)
                                .and_then(DataSegment::from_wasmparser)
                        })
                        .collect::<Result<_, _>>()?;
                }
                Payload::TableSection(table_section_reader) => {
                    tables = table_section_reader
                        .into_iter()
                        .map(|t| {
                            t.map_err(Error::from).map(|t| match t.init {
                                wasmparser::TableInit::RefNull => (t.ty, None),
                                wasmparser::TableInit::Expr(e) => (t.ty, Some(e)),
                            })
                        })
                        .collect::<Result<_, _>>()?;
                }
                Payload::MemorySection(memory_section_reader) => {
                    memories = memory_section_reader
                        .into_iter()
                        .collect::<Result<_, _>>()?;
                }
                Payload::FunctionSection(function_section_reader) => {
                    let temp: Vec<u32> = function_section_reader
                        .into_iter()
                        .collect::<Result<_, _>>()?;
                    functions.extend(temp.iter().map(|id| *id as TypeID));
                }
                Payload::GlobalSection(global_section_reader) => {
                    globals = global_section_reader
                        .into_iter()
                        .map(|g| Global::from_wasmparser(g?))
                        .collect::<Result<_, _>>()?;
                }
                Payload::ExportSection(export_section_reader) => {
                    for exp in export_section_reader.into_iter() {
                        exports.push(Export::from(exp?));
                    }
                }
                Payload::StartSection { func, range: _ } => {
                    if start.is_some() {
                        return Err(Error::MultipleStartSections);
                    }
                    start = Some(func as FunctionID);
                }
                Payload::ElementSection(element_section_reader) => {
                    for element in element_section_reader.into_iter() {
                        let element = element?;
                        let items = ElementItems::from_wasmparser(element.items.clone())?;
                        elements.push((ElementKind::from_wasmparser(element.kind)?, items));
                    }
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
                        .map(|(idx, val_type)| (*idx, DataType::from(*val_type)))
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
                        .map(|op| (op, InstrumentationFlag::default()))
                        .collect();
                    code_sections.push(Body {
                        locals,
                        num_locals,
                        instructions: instructions_bool.clone(),
                        num_instructions: instructions_bool.len(),
                        name: None,
                    });
                }
                Payload::CustomSection(custom_section_reader) => {
                    match custom_section_reader.as_known() {
                        wasmparser::KnownCustom::Name(name_section_reader) => {
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
                                    wasmparser::Name::Label(names) => {
                                        label_names = indirect_namemap_parser2encoder(names);
                                    }
                                    wasmparser::Name::Type(names) => {
                                        type_names = namemap_parser2encoder(names);
                                    }
                                    wasmparser::Name::Table(names) => {
                                        table_names = namemap_parser2encoder(names);
                                    }
                                    wasmparser::Name::Memory(names) => {
                                        memory_names = namemap_parser2encoder(names);
                                    }
                                    wasmparser::Name::Global(names) => {
                                        global_names = namemap_parser2encoder(names);
                                    }
                                    wasmparser::Name::Element(names) => {
                                        elem_names = namemap_parser2encoder(names);
                                    }
                                    wasmparser::Name::Data(names) => {
                                        data_names = namemap_parser2encoder(names);
                                    }
                                    wasmparser::Name::Field(names) => {
                                        field_names = indirect_namemap_parser2encoder(names);
                                    }
                                    wasmparser::Name::Tag(names) => {
                                        tag_names = namemap_parser2encoder(names);
                                    }
                                    wasmparser::Name::Unknown { .. } => {}
                                }
                            }
                        }
                        wasmparser::KnownCustom::Producers(producer_section_reader) => {
                            let field = producer_section_reader
                                .into_iter()
                                .next()
                                .unwrap()
                                .expect("producers field");
                            let _value = field
                                .values
                                .into_iter()
                                .collect::<Result<Vec<_>, _>>()
                                .expect("values");
                            custom_sections
                                .push((custom_section_reader.name(), custom_section_reader.data()));
                        }
                        _ => {
                            custom_sections
                                .push((custom_section_reader.name(), custom_section_reader.data()));
                        }
                    }
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

        // Add all the functions. First add all the imported functions as they have the first IDs
        let mut final_funcs = vec![];
        for (index, imp) in imports.iter().enumerate() {
            if let wasmparser::TypeRef::Func(u) = imp.ty {
                final_funcs.push(Function::new(
                    FuncKind::Import(ImportedFunction::new(index as ImportsID, u)),
                    Some(imp.name.parse().unwrap()),
                ))
            }
        }
        // Local Functions
        for (index, code_sec) in code_sections.iter().enumerate() {
            let mut args = vec![];
            for i in 0..types[functions[index] as usize].params.len() {
                args.push(i as LocalID);
            }
            final_funcs.push(Function::new(
                FuncKind::Local(LocalFunction::new(
                    functions[index],
                    (num_imported_functions + index) as FunctionID,
                    (*code_sec).clone(),
                    args,
                )),
                (*code_sec).clone().name,
            ));
        }

        Ok(Module {
            types: ModuleTypes::new(types),
            imports: ModuleImports::new(imports),
            functions: Functions::new(final_funcs, num_imported_functions, code_sections.len()),
            tables: ModuleTables::new(tables),
            memories,
            globals: ModuleGlobals::new(globals),
            exports: ModuleExports::new(exports),
            start,
            elements,
            data_count_section_exists: data_section_count.is_some(),
            // code_sections: code_sections.clone(),
            data,
            custom_sections: CustomSections::new(custom_sections),
            num_functions: code_sections.len(),
            num_imported_functions,
            module_name,
            local_names,
            type_names,
            table_names,
            elem_names,
            memory_names,
            global_names,
            data_names,
            field_names,
            tag_names,
            label_names,
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
    /// let mut module = Module::parse(&buff, false).unwrap();
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

        // initialize function name section
        let mut function_names = wasm_encoder::NameMap::new();
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
            for func in self.functions.iter() {
                if let FuncKind::Local(l) = func.kind() {
                    functions.function(l.ty_id);
                }
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
                    &export.name,
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

        if !self.num_functions > 0 {
            let mut code = wasm_encoder::CodeSection::new();
            for rel_func_idx in self.num_imported_functions..self.functions.len() {
                if let FuncKind::Import(..) = &self.functions.get_kind(rel_func_idx as FunctionID) {
                    continue;
                }
                let Body {
                    locals,
                    num_locals: _,
                    instructions,
                    num_instructions: _,
                    name,
                } = &self
                    .functions
                    .get(rel_func_idx as FunctionID)
                    .unwrap_local()
                    .body;

                let mut converted_locals = Vec::with_capacity(locals.len());
                for (c, ty) in locals {
                    converted_locals.push((*c, wasm_encoder::ValType::from(ty)));
                }
                let mut function = wasm_encoder::Function::new(converted_locals);
                for (op, instrument) in instructions {
                    if !instrument.has_instr() {
                        function.instruction(
                            &reencode
                                .instruction(op.clone())
                                .expect("Unable to convert Instruction"),
                        );
                    } else {
                        // this instruction has instrumentation, handle it!
                        let InstrumentationFlag {
                            before,
                            after,
                            alternate,
                            ..
                        } = instrument;

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
                                    .instruction(op.clone())
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
                if let Some(name) = name {
                    function_names.append(rel_func_idx as u32, name.as_str());
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

        if let Some(module_name) = &self.module_name {
            names.module(module_name);
        }
        names.functions(&function_names);
        names.locals(&self.local_names);
        names.labels(&self.label_names);
        names.types(&self.type_names);
        names.tables(&self.table_names);
        names.memories(&self.memory_names);
        names.globals(&self.global_names);
        names.elements(&self.elem_names);
        names.data(&self.data_names);
        names.fields(&self.field_names);
        names.tag(&self.tag_names);

        module.section(&names);

        // encode the rest of custom sections
        for section in self.custom_sections.iter() {
            module.section(&wasm_encoder::CustomSection {
                name: std::borrow::Cow::Borrowed(section.name),
                data: std::borrow::Cow::Borrowed(section.data),
            });
        }

        module
    }

    /// Add a new Data Segment to the module.
    /// Returns the index of the new Data Segment in the Data Section.
    pub fn add_data(&mut self, data: DataSegment) -> DataSegmentID {
        let index = self.data.len();
        self.data.push(data);
        index as DataSegmentID
    }

    /// Get the memory ID of a module. Does not support multiple memories
    pub fn get_memory_id(&self) -> Option<MemoryID> {
        if self.memories.len() > 1 {
            panic!("multiple memories unsupported")
        }

        if !self.memories.is_empty() {
            return Some(0 as MemoryID);
        }
        // module does not export a memory
        None
    }

    /// Add a new function to the module. Returns the index of the imported function. Panics if local functions are present as imported functions come first in the index space. Upto to the user to ensure imported functions are not added after local functions are already present.
    pub fn add_import_func(&mut self, module: String, name: String, ty_id: TypeID) -> ImportsID {
        if self.num_functions > 0 {
            panic!("Cannot add import function after adding a local function");
        }

        let index = self.imports.len();
        let import = Import {
            module: module.leak(),
            name: name.clone().leak(),
            ty: wasmparser::TypeRef::Func(ty_id),
            import_name: None,
        };
        self.imports.add(import);
        // Add to function as well as it has imported functions
        self.functions
            .add_import_func((self.imports.len() - 1) as ImportsID, ty_id, Some(name));
        self.num_imported_functions += 1;

        index as ImportsID
    }

    /// Delete an imported function
    pub fn delete_import_func(&mut self, import_id: ImportsID) {
        if import_id >= self.num_imported_functions as u32 {
            panic!("Invalid import function")
        }

        self.functions.delete(import_id);
        self.imports.delete(import_id);
        self.num_imported_functions -= 1;
    }

    /// Count number of imported function
    pub fn num_import_func(&self) -> u32 {
        self.num_imported_functions as u32
    }

    /// Set a function name to a function using its absolute index
    pub fn set_fn_name(&mut self, func_idx: FunctionID, name: String) {
        if func_idx < self.num_imported_functions as u32 {
            self.imports.set_name(name, func_idx);
        } else {
            self.functions.set_local_fn_name(func_idx, name);
        }
    }

    /// Create an empty Module
    pub fn new() -> Self {
        Module {
            types: ModuleTypes::new(vec![]),
            imports: ModuleImports::new(vec![]),
            functions: Functions::new(vec![], 0, 0),
            tables: ModuleTables::new(vec![]),
            memories: vec![],
            globals: ModuleGlobals::new(vec![]),
            exports: ModuleExports::new(vec![]),
            start: None,
            elements: vec![],
            data_count_section_exists: false,
            data: vec![],
            custom_sections: CustomSections::new(vec![]),
            num_functions: 0,
            num_imported_functions: 0,
            module_name: None,
            local_names: wasm_encoder::IndirectNameMap::new(),
            label_names: wasm_encoder::IndirectNameMap::new(),
            type_names: wasm_encoder::NameMap::new(),
            table_names: wasm_encoder::NameMap::new(),
            elem_names: wasm_encoder::NameMap::new(),
            memory_names: wasm_encoder::NameMap::new(),
            global_names: wasm_encoder::NameMap::new(),
            data_names: wasm_encoder::NameMap::new(),
            field_names: wasm_encoder::IndirectNameMap::new(),
            tag_names: wasm_encoder::NameMap::new(),
        }
    }
}

impl<'a> Default for Module<'a> {
    fn default() -> Self {
        Self::new()
    }
}
