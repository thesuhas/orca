use crate::convert::parser_to_internal;
use crate::error::Error;
use crate::wrappers::{
    convert_canon, convert_component_export, convert_component_instantiation_arg,
    convert_component_type, convert_component_val_type, convert_export, convert_heap_type,
    convert_instance_type, convert_instantiation_arg, convert_module_type_declaration,
    convert_params, convert_record_type, convert_results, convert_val_type, convert_variant_case,
    encode_core_type_subtype, process_alias, EncoderComponentExportKind, EncoderComponentTypeRef,
    EncoderComponentValType, EncoderEntityType, EncoderValType,
};
use wasm_encoder::reencode::Reencode;
use wasm_encoder::{ComponentAliasSection, ModuleSection};
use wasmparser::{
    CanonicalFunction, ComponentAlias, ComponentExport, ComponentImport, ComponentInstance,
    ComponentType, ComponentTypeDeclaration, CoreType, Export, GlobalType, Import, Instance,
    MemoryType, Operator, Parser, Payload, RefType, SubType, TableType, ValType,
};

pub struct Global<'a> {
    pub ty: GlobalType,
    pub init_expr: wasmparser::ConstExpr<'a>,
}

pub struct DataSegment<'a> {
    /// The kind of data segment.
    pub kind: DataSegmentKind<'a>,
    /// The data of the data segment.
    pub data: &'a [u8],
}

/// The kind of data segment.
#[derive(Debug, Clone)]
pub enum DataSegmentKind<'a> {
    /// The data segment is passive.
    Passive,
    /// The data segment is active.
    Active {
        /// The memory index for the data segment.
        memory_index: u32,
        /// The initialization operator for the data segment.
        offset_expr: wasmparser::ConstExpr<'a>,
    },
}

pub enum ElementKind<'a> {
    Passive,
    Active {
        table_index: Option<u32>,
        offset_expr: wasmparser::ConstExpr<'a>,
    },
    Declared,
}

pub enum ElementItems<'a> {
    Functions(Vec<u32>),
    ConstExprs {
        ty: RefType,
        exprs: Vec<wasmparser::ConstExpr<'a>>,
    },
}

pub struct Body<'a> {
    /// Local variables of the function, given as tuples of (# of locals, type).
    /// Note that these do not include the function parameters which are given
    /// indices before the locals. So if a function has 2 parameters and a local
    /// defined here then local indices 0 and 1 will refer to the parameters and
    /// index 2 will refer to the local here.
    pub locals: Vec<(u32, ValType)>,
    pub instructions: Vec<Operator<'a>>,
}

pub struct Module<'a> {
    pub types: Vec<SubType>,
    pub imports: Vec<Import<'a>>,
    /// Mapping from function index to type index.
    pub functions: Vec<u32>,
    /// Each table has a type and optional initialization expression.
    pub tables: Vec<(TableType, Option<wasmparser::ConstExpr<'a>>)>,
    pub memories: Vec<MemoryType>,
    pub globals: Vec<Global<'a>>,
    pub data: Vec<DataSegment<'a>>,
    pub data_count_section_exists: bool,
    pub exports: Vec<Export<'a>>,
    // Index of the start function.
    pub start: Option<u32>,
    pub elements: Vec<(ElementKind<'a>, ElementItems<'a>)>,
    pub code_sections: Vec<Body<'a>>,
    pub custom_sections: Vec<(&'a str, &'a [u8])>,
}

pub struct Component<'a> {
    /// Needs to contain:
    /// 1. Modules
    pub modules: Vec<Module<'a>>,
    /// 2. Alias
    pub alias: Vec<ComponentAlias<'a>>,
    /// 3. Types
    pub core_types: Vec<CoreType<'a>>,
    pub component_types: Vec<ComponentType<'a>>,
    /// 4. Import
    pub imports: Vec<ComponentImport<'a>>,
    /// 5. Export
    pub exports: Vec<ComponentExport<'a>>,
    /// 6. Instances
    pub instances: Vec<Instance<'a>>,
    pub component_instance: Vec<ComponentInstance<'a>>,
    /// 7. Canons
    pub canons: Vec<CanonicalFunction>,
    /// 8. Custom sections
    pub custom_sections: Vec<(&'a str, &'a [u8])>,
}

impl<'a> Component<'a> {
    pub fn parse(wasm: &'a [u8], enable_multi_memory: bool) -> Result<Self, Error> {
        let mut modules = vec![];
        let mut core_types = vec![];
        let mut component_types = vec![];
        let mut imports = vec![];
        let mut exports = vec![];
        let mut instances = vec![];
        let mut canons = vec![];
        let mut alias = vec![];
        let mut component_instance = vec![];
        let mut custom_sections = vec![];

        let parser = Parser::new(0);
        for payload in parser.parse_all(wasm) {
            let payload = payload?;
            // ComponentTypeSection(_) => { /* ... */ }
            // ComponentStartSection { .. } => { /* ... */ }
            match payload {
                Payload::ComponentImportSection(import_section_reader) => {
                    imports = import_section_reader
                        .into_iter()
                        .collect::<Result<_, _>>()?
                }
                Payload::ComponentExportSection(export_section_reader) => {
                    exports = export_section_reader
                        .into_iter()
                        .collect::<Result<_, _>>()?;
                }
                Payload::InstanceSection(instance_section_reader) => {
                    instances = instance_section_reader
                        .into_iter()
                        .collect::<Result<_, _>>()?;
                }
                Payload::CoreTypeSection(core_type_reader) => {
                    core_types = core_type_reader.into_iter().collect::<Result<_, _>>()?;
                }
                Payload::ComponentTypeSection(component_type_reader) => {
                    component_types = component_type_reader
                        .into_iter()
                        .collect::<Result<_, _>>()?;
                }
                Payload::ComponentInstanceSection(component_instances) => {
                    component_instance =
                        component_instances.into_iter().collect::<Result<_, _>>()?;
                }
                Payload::ComponentAliasSection(alias_reader) => {
                    alias = alias_reader.into_iter().collect::<Result<_, _>>()?;
                }
                Payload::ComponentCanonicalSection(canon_reader) => {
                    canons = canon_reader.into_iter().collect::<Result<_, _>>()?;
                }
                Payload::ModuleSection {
                    parser,
                    unchecked_range,
                } => {
                    modules.push(Module::parse(
                        &wasm[unchecked_range],
                        enable_multi_memory,
                        parser,
                    )?);
                }
                Payload::ComponentSection {
                    parser: _,
                    unchecked_range: _,
                } => {}
                Payload::CustomSection(custom_section_reader) => {
                    custom_sections
                        .push((custom_section_reader.name(), custom_section_reader.data()));
                }
                Payload::UnknownSection {
                    id,
                    contents: _,
                    range: _,
                } => return Err(Error::UnknownSection { section_id: id }),
                _ => {}
            }
        }
        Ok(Component {
            modules,
            alias,
            core_types,
            component_types,
            imports,
            exports,
            instances,
            component_instance,
            canons,
            custom_sections,
        })
    }

    pub fn encode(self) -> Result<Vec<u8>, Error> {
        let mut component = wasm_encoder::Component::new();

        // Module parsing
        if !self.modules.is_empty() {
            // Parse each module
            for m in self.modules {
                component.section(&ModuleSection(&m.encode()?));
            }
        }

        // Alias parsing
        if !self.alias.is_empty() {
            let mut alias = ComponentAliasSection::new();
            for a in self.alias {
                alias.alias(process_alias(a));
            }
            component.section(&alias);
        }

        // Core Types
        if !self.core_types.is_empty() {
            let mut type_section = wasm_encoder::CoreTypeSection::new();
            for ty in self.core_types {
                match ty {
                    CoreType::Sub(subtype) => {
                        let enc = type_section.ty();
                        encode_core_type_subtype(enc, subtype);
                    }
                    CoreType::Module(module) => {
                        for m in &*module {
                            type_section.module(&convert_module_type_declaration((*m).clone()));
                        }
                    }
                }
            }
            component.section(&type_section);
        }

        // Component Types
        if !self.component_types.is_empty() {
            let mut component_ty_section = wasm_encoder::ComponentTypeSection::new();
            for ty in self.component_types {
                match ty {
                    ComponentType::Defined(comp_ty) => {
                        let enc = component_ty_section.defined_type();
                        match comp_ty {
                            wasmparser::ComponentDefinedType::Primitive(p) => {
                                enc.primitive(wasm_encoder::PrimitiveValType::from(p))
                            }
                            wasmparser::ComponentDefinedType::Record(record) => {
                                enc.record(record.into_vec().into_iter().map(convert_record_type));
                            }
                            wasmparser::ComponentDefinedType::Variant(variant) => enc
                                .variant(variant.into_vec().into_iter().map(convert_variant_case)),
                            wasmparser::ComponentDefinedType::List(l) => {
                                enc.list(EncoderComponentValType::from(l).ret_original())
                            }
                            wasmparser::ComponentDefinedType::Tuple(tup) => enc
                                .tuple(tup.into_vec().into_iter().map(convert_component_val_type)),
                            wasmparser::ComponentDefinedType::Flags(flags) => {
                                enc.flags(flags.into_vec().into_iter())
                            }
                            wasmparser::ComponentDefinedType::Enum(en) => {
                                enc.enum_type(en.into_vec().into_iter())
                            }
                            wasmparser::ComponentDefinedType::Option(opt) => {
                                enc.option(convert_component_val_type(opt))
                            }
                            wasmparser::ComponentDefinedType::Result { ok, err } => enc.result(
                                match ok {
                                    None => None,
                                    Some(val) => Some(convert_component_val_type(val)),
                                },
                                match err {
                                    None => None,
                                    Some(e) => Some(convert_component_val_type(e)),
                                },
                            ),
                            wasmparser::ComponentDefinedType::Own(u) => enc.own(u),
                            wasmparser::ComponentDefinedType::Borrow(u) => enc.borrow(u),
                        }
                    }
                    ComponentType::Func(func_ty) => {
                        let mut enc = component_ty_section.function();
                        enc.params(func_ty.params.into_vec().into_iter().map(convert_params));
                        convert_results(func_ty.results, enc);
                    }
                    ComponentType::Component(comp) => {
                        let mut new_comp = wasm_encoder::ComponentType::new();
                        for c in comp.into_vec().into_iter() {
                            match c {
                                ComponentTypeDeclaration::CoreType(core) => match core {
                                    CoreType::Sub(sub) => {
                                        let enc = new_comp.core_type();
                                        encode_core_type_subtype(enc, sub);
                                    }
                                    CoreType::Module(module) => {
                                        for m in &*module {
                                            let enc = new_comp.core_type();
                                            enc.module(&convert_module_type_declaration(
                                                (*m).clone(),
                                            ));
                                        }
                                    }
                                },
                                ComponentTypeDeclaration::Type(typ) => {
                                    let enc = new_comp.ty();
                                    convert_component_type(typ, enc);
                                }
                                ComponentTypeDeclaration::Alias(a) => {
                                    new_comp.alias(process_alias(a));
                                }
                                ComponentTypeDeclaration::Export { name, ty } => {
                                    new_comp.export(
                                        name.0,
                                        EncoderComponentTypeRef::from(ty).ret_original(),
                                    );
                                }
                                ComponentTypeDeclaration::Import(imp) => {
                                    new_comp.import(
                                        imp.name.0,
                                        EncoderComponentTypeRef::from(imp.ty).ret_original(),
                                    );
                                }
                            }
                        }
                        component_ty_section.component(&new_comp);
                    }
                    ComponentType::Instance(inst) => {
                        for i in inst.into_vec().into_iter() {
                            component_ty_section.instance(&convert_instance_type(i));
                        }
                    }
                    ComponentType::Resource { rep, dtor } => {
                        component_ty_section
                            .resource(EncoderValType::from(rep).ret_original(), dtor);
                    }
                }
            }
            component.section(&component_ty_section);
        }

        // Import parsing
        if !self.imports.is_empty() {
            let mut imports = wasm_encoder::ComponentImportSection::new();
            for imp in self.imports {
                imports.import(
                    imp.name.0,
                    EncoderComponentTypeRef::from(imp.ty).ret_original(),
                );
            }
            component.section(&imports);
        }

        // Export parsing
        if !self.exports.is_empty() {
            let mut exports = wasm_encoder::ComponentExportSection::new();
            for exp in self.exports {
                exports.export(
                    exp.name.0,
                    EncoderComponentExportKind::from(exp.kind).ret_original(),
                    exp.index,
                    match exp.ty {
                        None => None,
                        Some(ty) => Some(EncoderComponentTypeRef::from(ty).ret_original()),
                    },
                );
            }
            component.section(&exports);
        }

        // Component Instance parsing
        if !self.component_instance.is_empty() {
            let mut instances = wasm_encoder::ComponentInstanceSection::new();
            for instance in self.component_instance {
                match instance {
                    ComponentInstance::Instantiate {
                        component_index,
                        args,
                    } => {
                        instances.instantiate(
                            component_index,
                            args.into_vec()
                                .into_iter()
                                .map(convert_component_instantiation_arg),
                        );
                    }
                    ComponentInstance::FromExports(export) => {
                        instances.export_items(
                            export.into_vec().into_iter().map(convert_component_export),
                        );
                    }
                }
            }
            component.section(&instances);
        }

        // Core Instance Parsing
        if !self.instances.is_empty() {
            let mut instances = wasm_encoder::InstanceSection::new();
            for instance in self.instances {
                match instance {
                    Instance::Instantiate { module_index, args } => {
                        instances.instantiate(
                            module_index,
                            args.into_vec().into_iter().map(convert_instantiation_arg),
                        );
                    }
                    Instance::FromExports(exports) => {
                        instances.export_items(exports.into_vec().into_iter().map(convert_export));
                    }
                }
            }
            component.section(&instances);
        }

        // Canons parsing
        if !self.canons.is_empty() {
            let mut canon_sec = wasm_encoder::CanonicalFunctionSection::new();
            for canon in self.canons {
                match canon {
                    CanonicalFunction::Lift {
                        core_func_index,
                        type_index,
                        options,
                    } => {
                        canon_sec.lift(
                            core_func_index,
                            type_index,
                            options.into_vec().into_iter().map(convert_canon),
                        );
                    }
                    CanonicalFunction::Lower {
                        func_index,
                        options,
                    } => {
                        canon_sec.lower(
                            func_index,
                            options.into_vec().into_iter().map(convert_canon),
                        );
                    }
                    CanonicalFunction::ResourceNew { resource } => {
                        canon_sec.resource_new(resource);
                    }
                    CanonicalFunction::ResourceDrop { resource } => {
                        canon_sec.resource_rep(resource);
                    }
                    CanonicalFunction::ResourceRep { resource } => {
                        canon_sec.resource_rep(resource);
                    }
                }
            }
            component.section(&canon_sec);
        }

        // Custom sections
        for (name, data) in self.custom_sections {
            component.section(&wasm_encoder::CustomSection {
                name: std::borrow::Cow::Borrowed(name),
                data: std::borrow::Cow::Borrowed(data),
            });
        }
        Ok(component.finish())
    }
}

impl<'a> Module<'a> {
    pub fn parse_only_module(wasm: &'a [u8], enable_multi_memory: bool) -> Result<Self, Error> {
        let parser = Parser::new(0);
        Module::parse(wasm, enable_multi_memory, parser)
    }

    pub fn parse(wasm: &'a [u8], enable_multi_memory: bool, parser: Parser) -> Result<Self, Error> {
        // let parser = Parser::new(0);
        let mut imports = vec![];
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
        for payload in parser.parse_all(wasm) {
            let payload = payload?;
            match payload {
                Payload::ImportSection(import_section_reader) => {
                    imports = import_section_reader
                        .into_iter()
                        .collect::<Result<_, _>>()?;
                }
                Payload::TypeSection(type_section_reader) => {
                    for rec_group in type_section_reader.into_iter() {
                        types.extend(rec_group?.into_types());
                    }
                }
                Payload::DataSection(data_section_reader) => {
                    data = data_section_reader
                        .into_iter()
                        .map(|sec| {
                            sec.map_err(Error::from)
                                .and_then(parser_to_internal::data_segment)
                        })
                        .collect::<Result<_, _>>()?;
                }
                Payload::TableSection(table_section_reader) => {
                    tables = table_section_reader
                        .into_iter()
                        .map(|t| {
                            t.map_err(Error::from).and_then(|t| match t.init {
                                wasmparser::TableInit::RefNull => Ok((t.ty, None)),
                                wasmparser::TableInit::Expr(e) => Ok((t.ty, Some(e))),
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
                    functions = function_section_reader
                        .into_iter()
                        .collect::<Result<_, _>>()?;
                }
                Payload::GlobalSection(global_section_reader) => {
                    globals = global_section_reader
                        .into_iter()
                        .map(|g| parser_to_internal::global(g?))
                        .collect::<Result<_, _>>()?;
                }
                Payload::ExportSection(export_section_reader) => {
                    exports = export_section_reader
                        .into_iter()
                        .collect::<Result<_, _>>()?;
                }
                Payload::StartSection { func, range: _ } => {
                    if start.is_some() {
                        return Err(Error::MultipleStartSections);
                    }
                    start = Some(func);
                }
                Payload::ElementSection(element_section_reader) => {
                    for element in element_section_reader.into_iter() {
                        let element = element?;
                        let items = parser_to_internal::element_items(element.items.clone())?;
                        elements.push((parser_to_internal::element_kind(element.kind)?, items));
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
                    code_sections.push(Body {
                        locals,
                        instructions,
                    });
                }
                Payload::CustomSection(custom_section_reader) => {
                    custom_sections
                        .push((custom_section_reader.name(), custom_section_reader.data()));
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
            code_sections,
            data,
            custom_sections,
        })
    }

    pub fn encode_only_module(self) -> Result<Vec<u8>, Error> {
        Ok(self.encode().unwrap().finish())
    }

    pub fn encode(self) -> Result<wasm_encoder::Module, Error> {
        let mut module = wasm_encoder::Module::new();
        let mut reencode = wasm_encoder::reencode::RoundtripReencoder;

        if !self.types.is_empty() {
            let mut types = wasm_encoder::TypeSection::new();
            for subtype in self.types {
                types.subtype(&wasm_encoder::SubType::try_from(subtype.clone()).unwrap());
            }
            module.section(&types);
        }

        if !self.imports.is_empty() {
            let mut imports = wasm_encoder::ImportSection::new();
            for import in self.imports {
                imports.import(
                    import.module,
                    import.name,
                    EncoderEntityType::from(import.ty).ret_original(),
                );
            }
            module.section(&imports);
        }

        if !self.functions.is_empty() {
            let mut functions = wasm_encoder::FunctionSection::new();
            for type_index in self.functions {
                functions.function(type_index);
            }
            module.section(&functions);
        }

        if !self.tables.is_empty() {
            let mut tables = wasm_encoder::TableSection::new();
            for (table_ty, init) in self.tables {
                let table_ty = wasm_encoder::TableType {
                    element_type: wasm_encoder::RefType {
                        nullable: table_ty.element_type.is_nullable(),
                        heap_type: convert_heap_type(table_ty.element_type.heap_type()),
                    },
                    table64: table_ty.table64,
                    minimum: table_ty.initial, // TODO - Check if this maps
                    maximum: table_ty.maximum,
                };
                match init {
                    None => tables.table(table_ty),
                    Some(const_expr) => tables.table_with_init(
                        table_ty,
                        &reencode
                            .const_expr(const_expr)
                            .expect("Error in Converting Const Expr"),
                    ),
                };
            }
            module.section(&tables);
        }

        if !self.memories.is_empty() {
            let mut memories = wasm_encoder::MemorySection::new();
            for memory in self.memories {
                memories.memory(wasm_encoder::MemoryType::from(memory));
            }
            module.section(&memories);
        }

        if !self.globals.is_empty() {
            let mut globals = wasm_encoder::GlobalSection::new();
            for global in self.globals {
                globals.global(
                    wasm_encoder::GlobalType {
                        val_type: convert_val_type(&global.ty.content_type),
                        mutable: global.ty.mutable,
                        shared: global.ty.shared,
                    },
                    &reencode
                        .const_expr(global.init_expr)
                        .expect("Error unpacking globals"),
                );
            }
            module.section(&globals);
        }

        if !self.exports.is_empty() {
            let mut exports = wasm_encoder::ExportSection::new();
            for export in self.exports {
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
            for (kind, items) in self.elements {
                temp_const_exprs.clear();
                let element_items = match &items {
                    ElementItems::Functions(funcs) => wasm_encoder::Elements::Functions(funcs),
                    ElementItems::ConstExprs { ty, exprs } => {
                        temp_const_exprs.reserve(exprs.len());
                        for e in exprs {
                            temp_const_exprs.push(
                                reencode
                                    .const_expr((*e).clone())
                                    .expect("Unable to convert element constant expr"),
                            );
                        }
                        wasm_encoder::Elements::Expressions(
                            wasm_encoder::RefType {
                                nullable: ty.is_nullable(),
                                heap_type: convert_heap_type(ty.heap_type()),
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
                            table_index,
                            &reencode
                                .const_expr(offset_expr)
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
            for Body {
                locals,
                instructions,
            } in self.code_sections
            {
                let mut converted_locals = Vec::with_capacity(locals.len());
                for (c, t) in locals {
                    converted_locals.push((c, EncoderValType::from(t).ret_original()));
                }
                let mut function = wasm_encoder::Function::new(converted_locals);
                for op in instructions {
                    function.instruction(
                        &reencode
                            .instruction(op)
                            .expect("Unable to convert Instruction"),
                    );
                }
                code.function(&function);
            }
            module.section(&code);
        }

        if !self.data.is_empty() {
            let mut data = wasm_encoder::DataSection::new();
            for segment in self.data {
                let segment_data = segment.data.iter().copied();
                match segment.kind {
                    DataSegmentKind::Passive => data.passive(segment_data),
                    DataSegmentKind::Active {
                        memory_index,
                        offset_expr,
                    } => data.active(
                        memory_index,
                        &reencode
                            .const_expr(offset_expr)
                            .expect("Data segment offset expr"),
                        segment_data,
                    ),
                };
            }
            module.section(&data);
        }

        for (name, data) in self.custom_sections {
            module.section(&wasm_encoder::CustomSection {
                name: std::borrow::Cow::Borrowed(name),
                data: std::borrow::Cow::Borrowed(data),
            });
        }

        Ok(module)
    }
}
