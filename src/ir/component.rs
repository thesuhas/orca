use crate::error::Error;
use crate::ir::module::Module;
use crate::ir::types::InstrumentType;
use crate::ir::wrappers::{
    compare_operator_for_inject, compare_operator_instr_ty, convert_canon,
    convert_component_export, convert_component_instantiation_arg, convert_component_type,
    convert_component_val_type, convert_export, convert_instance_type, convert_instantiation_arg,
    convert_module_type_declaration, convert_params, convert_record_type, convert_results,
    convert_variant_case, encode_core_type_subtype, process_alias, EncoderComponentExportKind,
    EncoderComponentTypeRef, EncoderComponentValType, EncoderValType,
};
use wasm_encoder::{ComponentAliasSection, ModuleSection};
use wasmparser::{
    CanonicalFunction, ComponentAlias, ComponentExport, ComponentImport, ComponentInstance,
    ComponentType, ComponentTypeDeclaration, CoreType, Instance, Operator, Parser, Payload,
};

#[derive(Debug, Clone)]
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

    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut component = wasm_encoder::Component::new();

        // Module parsing
        if !self.modules.is_empty() {
            // Parse each module
            for m in self.modules.iter() {
                component.section(&ModuleSection(&m.encode()?));
            }
        }

        // Alias parsing
        if !self.alias.is_empty() {
            let mut alias = ComponentAliasSection::new();
            for a in self.alias.iter() {
                alias.alias(process_alias(a));
            }
            component.section(&alias);
        }

        // Core Types
        if !self.core_types.is_empty() {
            let mut type_section = wasm_encoder::CoreTypeSection::new();
            for ty in self.core_types.iter() {
                match ty {
                    CoreType::Sub(subtype) => {
                        let enc = type_section.ty();
                        encode_core_type_subtype(enc, subtype);
                    }
                    CoreType::Module(module) => {
                        for m in module.iter() {
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
            for ty in self.component_types.iter() {
                match ty {
                    ComponentType::Defined(comp_ty) => {
                        let enc = component_ty_section.defined_type();
                        match comp_ty {
                            wasmparser::ComponentDefinedType::Primitive(p) => {
                                enc.primitive(wasm_encoder::PrimitiveValType::from(*p))
                            }
                            wasmparser::ComponentDefinedType::Record(record) => {
                                enc.record(record.iter().map(convert_record_type));
                            }
                            wasmparser::ComponentDefinedType::Variant(variant) => {
                                enc.variant(variant.iter().map(convert_variant_case))
                            }
                            wasmparser::ComponentDefinedType::List(l) => {
                                enc.list(EncoderComponentValType::from(*l).ret_original())
                            }
                            wasmparser::ComponentDefinedType::Tuple(tup) => enc.tuple(
                                tup.clone()
                                    .into_vec()
                                    .into_iter()
                                    .map(convert_component_val_type),
                            ),
                            wasmparser::ComponentDefinedType::Flags(flags) => {
                                enc.flags(flags.clone().into_vec().into_iter())
                            }
                            wasmparser::ComponentDefinedType::Enum(en) => {
                                enc.enum_type(en.clone().into_vec().into_iter())
                            }
                            wasmparser::ComponentDefinedType::Option(opt) => {
                                enc.option(convert_component_val_type(*opt))
                            }
                            wasmparser::ComponentDefinedType::Result { ok, err } => enc.result(
                                ok.map(convert_component_val_type),
                                err.map(convert_component_val_type),
                            ),
                            wasmparser::ComponentDefinedType::Own(u) => enc.own(*u),
                            wasmparser::ComponentDefinedType::Borrow(u) => enc.borrow(*u),
                        }
                    }
                    ComponentType::Func(func_ty) => {
                        let mut enc = component_ty_section.function();
                        enc.params(
                            func_ty
                                .params
                                .iter()
                                .map(|p: &(&str, wasmparser::ComponentValType)| convert_params(*p)),
                        );
                        convert_results(func_ty.results.clone(), enc);
                    }
                    ComponentType::Component(comp) => {
                        let mut new_comp = wasm_encoder::ComponentType::new();
                        for c in comp.iter() {
                            match c {
                                ComponentTypeDeclaration::CoreType(core) => match core {
                                    CoreType::Sub(sub) => {
                                        let enc = new_comp.core_type();
                                        encode_core_type_subtype(enc, sub);
                                    }
                                    CoreType::Module(module) => {
                                        for m in module.iter() {
                                            let enc = new_comp.core_type();
                                            enc.module(&convert_module_type_declaration(
                                                (*m).clone(),
                                            ));
                                        }
                                    }
                                },
                                ComponentTypeDeclaration::Type(typ) => {
                                    let enc = new_comp.ty();
                                    convert_component_type((*typ).clone(), enc);
                                }
                                ComponentTypeDeclaration::Alias(a) => {
                                    new_comp.alias(process_alias(a));
                                }
                                ComponentTypeDeclaration::Export { name, ty } => {
                                    new_comp.export(
                                        name.0,
                                        EncoderComponentTypeRef::from(*ty).ret_original(),
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
                        for i in inst.iter() {
                            component_ty_section.instance(&convert_instance_type((*i).clone()));
                        }
                    }
                    ComponentType::Resource { rep, dtor } => {
                        component_ty_section
                            .resource(EncoderValType::from(*rep).ret_original(), *dtor);
                    }
                }
            }
            component.section(&component_ty_section);
        }

        // Import parsing
        if !self.imports.is_empty() {
            let mut imports = wasm_encoder::ComponentImportSection::new();
            for imp in self.imports.iter() {
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
            for exp in self.exports.iter() {
                exports.export(
                    exp.name.0,
                    EncoderComponentExportKind::from(exp.kind).ret_original(),
                    exp.index,
                    exp.ty
                        .map(|ty| EncoderComponentTypeRef::from(ty).ret_original()),
                );
            }
            component.section(&exports);
        }

        // Component Instance parsing
        if !self.component_instance.is_empty() {
            let mut instances = wasm_encoder::ComponentInstanceSection::new();
            for instance in self.component_instance.iter() {
                match instance {
                    ComponentInstance::Instantiate {
                        component_index,
                        args,
                    } => {
                        instances.instantiate(
                            *component_index,
                            args.iter().map(convert_component_instantiation_arg),
                        );
                    }
                    ComponentInstance::FromExports(export) => {
                        instances.export_items(export.iter().map(convert_component_export));
                    }
                }
            }
            component.section(&instances);
        }

        // Core Instance Parsing
        if !self.instances.is_empty() {
            let mut instances = wasm_encoder::InstanceSection::new();
            for instance in self.instances.iter() {
                match instance {
                    Instance::Instantiate { module_index, args } => {
                        instances
                            .instantiate(*module_index, args.iter().map(convert_instantiation_arg));
                    }
                    Instance::FromExports(exports) => {
                        instances.export_items(exports.iter().map(convert_export));
                    }
                }
            }
            component.section(&instances);
        }

        // Canons parsing
        if !self.canons.is_empty() {
            let mut canon_sec = wasm_encoder::CanonicalFunctionSection::new();
            for canon in self.canons.iter() {
                match canon {
                    CanonicalFunction::Lift {
                        core_func_index,
                        type_index,
                        options,
                    } => {
                        canon_sec.lift(
                            *core_func_index,
                            *type_index,
                            options.iter().map(convert_canon),
                        );
                    }
                    CanonicalFunction::Lower {
                        func_index,
                        options,
                    } => {
                        canon_sec.lower(*func_index, options.iter().map(convert_canon));
                    }
                    CanonicalFunction::ResourceNew { resource } => {
                        canon_sec.resource_new(*resource);
                    }
                    CanonicalFunction::ResourceDrop { resource } => {
                        canon_sec.resource_rep(*resource);
                    }
                    CanonicalFunction::ResourceRep { resource } => {
                        canon_sec.resource_rep(*resource);
                    }
                }
            }
            component.section(&canon_sec);
        }

        // Custom sections
        for (name, data) in self.custom_sections.iter() {
            component.section(&wasm_encoder::CustomSection {
                name: std::borrow::Cow::Borrowed(name),
                data: std::borrow::Cow::Borrowed(data),
            });
        }
        Ok(component.finish())
    }

    /// Print every instruction
    pub fn visitor(&self) {
        // This function is responsible for visiting every instruction
        for (index, module) in self.modules.iter().enumerate() {
            println!("Entered Module: {}", index);
            (*module).clone().visitor();
        }
    }

    /// flip the instrument type according to the instruction of interest
    pub fn mark_as_instrument(&mut self, interest_instr: Vec<(Operator, InstrumentType)>) {
        // This function is responsible for visiting every instruction
        for (mod_idx, module) in self.modules.iter_mut().enumerate() {
            println!("Entered Module: {}", mod_idx);
            for (func_idx, body) in module.code_sections.iter_mut().enumerate() {
                println!("Entered Function: {}", func_idx);
                // Each function index should match to a code section
                // for (local_idx, local_ty) in body.locals.iter() {
                //     println!("Local {}: {}", local_idx, local_ty);
                // }
                for (instr_idx, (instr, ref mut instrumented)) in
                    body.instructions.iter_mut().enumerate()
                {
                    println!(" {}: {:?}, {}", instr_idx, instr, instrumented);
                    *instrumented = compare_operator_instr_ty(interest_instr.clone(), instr);
                }
            }
        }
    }

    pub fn add_instrumentation(&mut self, code_injections: Vec<(Operator<'a>, Operator<'a>)>) {
        for (_module_idx, module) in self.modules.iter_mut().enumerate() {
            for (_fun_idx, body) in module.code_sections.iter_mut().enumerate() {
                // Each function index should match to a code section
                for (local_idx, local_ty) in body.locals.iter() {
                    println!("Local {}: {}", local_idx, local_ty);
                }

                let mut changes = Vec::new();

                for (idx, (instr, instrumented)) in body.instructions.iter_mut().enumerate() {
                    if *instrumented != InstrumentType::NotInstrumented {
                        match compare_operator_for_inject(code_injections.clone(), instr.clone()) {
                            Some(inject) => {
                                if *instrumented == InstrumentType::InstrumentAlternate {
                                    // Replace the value
                                    changes.push((idx, inject, InstrumentType::NotInstrumented));
                                } else if *instrumented == InstrumentType::InstrumentBefore {
                                    *instrumented = InstrumentType::NotInstrumented;
                                    changes.push((idx, inject, InstrumentType::NotInstrumented));
                                } else {
                                    *instrumented = InstrumentType::NotInstrumented;
                                    changes.push((
                                        idx + 1,
                                        inject,
                                        InstrumentType::NotInstrumented,
                                    ));
                                }
                            }
                            None => {
                                // If nothing matches, reset instrumentation
                                *instrumented = InstrumentType::NotInstrumented;
                            }
                        }
                    }
                }

                // Apply changes
                let mut offset = 0;
                for (idx, op, instr) in changes {
                    if instr == InstrumentType::InstrumentAlternate {
                        body.instructions[idx + offset] = (op, instr);
                    } else {
                        body.instructions
                            .insert(idx + offset, (op, InstrumentType::NotInstrumented));
                        offset += 1;
                    }
                }
            }
        }
    }
}
