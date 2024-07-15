//! Intermediate Representation of a wasm component.

use crate::error::Error;
use crate::ir::module::Module;
use crate::ir::types::Global;
use crate::ir::wrappers::{
    convert_component_type, convert_instance_type, convert_module_type_declaration,
    convert_results, encode_core_type_subtype, process_alias,
};
use wasm_encoder::reencode::Reencode;
use wasm_encoder::{ComponentAliasSection, ModuleArg, ModuleSection};
use wasmparser::{
    CanonicalFunction, ComponentAlias, ComponentExport, ComponentImport, ComponentInstance,
    ComponentType, ComponentTypeDeclaration, CoreType, Instance, Parser, Payload,
};

#[derive(Debug, Clone)]
/// Intermediate Representation of a wasm component.
pub struct Component<'a> {
    /// Modules
    pub modules: Vec<Module<'a>>,
    ///Alias
    pub alias: Vec<ComponentAlias<'a>>,
    /// Core Types
    pub core_types: Vec<CoreType<'a>>,
    /// Component Types
    pub component_types: Vec<ComponentType<'a>>,
    /// Imports
    pub imports: Vec<ComponentImport<'a>>,
    /// Exports
    pub exports: Vec<ComponentExport<'a>>,
    /// Core Instances
    pub instances: Vec<Instance<'a>>,
    /// Component Instances
    pub component_instance: Vec<ComponentInstance<'a>>,
    /// Canons
    pub canons: Vec<CanonicalFunction>,
    /// Custom sections
    pub custom_sections: Vec<(&'a str, &'a [u8])>,
    /// Number of modules
    pub num_modules: usize,
}

impl Default for Component<'_> {
    fn default() -> Self {
        Component::new()
    }
}

impl<'a> Component<'a> {
    pub fn new() -> Self {
        Component {
            modules: vec![],
            alias: vec![],
            core_types: vec![],
            component_types: vec![],
            imports: vec![],
            exports: vec![],
            instances: vec![],
            component_instance: vec![],
            canons: vec![],
            custom_sections: vec![],
            num_modules: 0,
        }
    }

    pub fn add_module(&mut self, module: Module<'a>) {
        self.modules.push(module);
        self.num_modules += 1;
    }

    pub fn add_globals(&mut self, global: Global, module_idx: usize) -> u32 {
        self.modules[module_idx].add_global(global)
    }

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
                    imports.append(
                        &mut import_section_reader
                            .into_iter()
                            .collect::<Result<_, _>>()?,
                    );
                }
                Payload::ComponentExportSection(export_section_reader) => {
                    exports.append(
                        &mut export_section_reader
                            .into_iter()
                            .collect::<Result<_, _>>()?,
                    );
                }
                Payload::InstanceSection(instance_section_reader) => {
                    instances.append(
                        &mut instance_section_reader
                            .into_iter()
                            .collect::<Result<_, _>>()?,
                    );
                }
                Payload::CoreTypeSection(core_type_reader) => {
                    core_types.append(&mut core_type_reader.into_iter().collect::<Result<_, _>>()?);
                }
                Payload::ComponentTypeSection(component_type_reader) => {
                    component_types.append(
                        &mut component_type_reader
                            .into_iter()
                            .collect::<Result<_, _>>()?,
                    );
                }
                Payload::ComponentInstanceSection(component_instances) => {
                    component_instance
                        .append(&mut component_instances.into_iter().collect::<Result<_, _>>()?);
                }
                Payload::ComponentAliasSection(alias_reader) => {
                    alias.append(&mut alias_reader.into_iter().collect::<Result<_, _>>()?);
                }
                Payload::ComponentCanonicalSection(canon_reader) => {
                    canons.append(&mut canon_reader.into_iter().collect::<Result<_, _>>()?);
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
            modules: modules.clone(),
            alias,
            core_types,
            component_types,
            imports,
            exports,
            instances,
            component_instance,
            canons,
            custom_sections,
            num_modules: modules.len(),
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut component = wasm_encoder::Component::new();
        let mut reencode = wasm_encoder::reencode::RoundtripReencoder;

        // Module parsing
        if !self.modules.is_empty() {
            // Parse each module
            for m in self.modules.iter() {
                component.section(&ModuleSection(&m.encode()));
            }
        }

        // Core Types
        if !self.core_types.is_empty() {
            let mut type_section = wasm_encoder::CoreTypeSection::new();
            for ty in self.core_types.iter() {
                match ty {
                    CoreType::Sub(subtype) => {
                        let enc = type_section.ty();
                        encode_core_type_subtype(enc, subtype, &mut reencode);
                    }
                    CoreType::Module(module) => {
                        for m in module.iter() {
                            type_section.module(&convert_module_type_declaration(
                                (*m).clone(),
                                &mut reencode,
                            ));
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
                                enc.record(record.iter().map(|record| {
                                    (record.0, reencode.component_val_type(record.1))
                                }));
                            }
                            wasmparser::ComponentDefinedType::Variant(variant) => {
                                enc.variant(variant.iter().map(|variant| {
                                    (
                                        variant.name,
                                        variant.ty.map(|ty| reencode.component_val_type(ty)),
                                        variant.refines,
                                    )
                                }))
                            }
                            wasmparser::ComponentDefinedType::List(l) => {
                                enc.list(reencode.component_val_type(*l))
                            }
                            wasmparser::ComponentDefinedType::Tuple(tup) => enc.tuple(
                                tup.clone()
                                    .into_vec()
                                    .into_iter()
                                    .map(|val_type| reencode.component_val_type(val_type)),
                            ),
                            wasmparser::ComponentDefinedType::Flags(flags) => {
                                enc.flags(flags.clone().into_vec().into_iter())
                            }
                            wasmparser::ComponentDefinedType::Enum(en) => {
                                enc.enum_type(en.clone().into_vec().into_iter())
                            }
                            wasmparser::ComponentDefinedType::Option(opt) => {
                                enc.option(reencode.component_val_type(*opt))
                            }
                            wasmparser::ComponentDefinedType::Result { ok, err } => enc.result(
                                ok.map(|val_type| reencode.component_val_type(val_type)),
                                err.map(|val_type| reencode.component_val_type(val_type)),
                            ),
                            wasmparser::ComponentDefinedType::Own(u) => enc.own(*u),
                            wasmparser::ComponentDefinedType::Borrow(u) => enc.borrow(*u),
                        }
                    }
                    ComponentType::Func(func_ty) => {
                        let mut enc = component_ty_section.function();
                        enc.params(func_ty.params.iter().map(
                            |p: &(&str, wasmparser::ComponentValType)| {
                                (p.0, reencode.component_val_type(p.1))
                            },
                        ));
                        convert_results(func_ty.results.clone(), enc, &mut reencode);
                    }
                    ComponentType::Component(comp) => {
                        let mut new_comp = wasm_encoder::ComponentType::new();
                        for c in comp.iter() {
                            match c {
                                ComponentTypeDeclaration::CoreType(core) => match core {
                                    CoreType::Sub(sub) => {
                                        let enc = new_comp.core_type();
                                        encode_core_type_subtype(enc, sub, &mut reencode);
                                    }
                                    CoreType::Module(module) => {
                                        for m in module.iter() {
                                            let enc = new_comp.core_type();
                                            enc.module(&convert_module_type_declaration(
                                                (*m).clone(),
                                                &mut reencode,
                                            ));
                                        }
                                    }
                                },
                                ComponentTypeDeclaration::Type(typ) => {
                                    let enc = new_comp.ty();
                                    convert_component_type((*typ).clone(), enc, &mut reencode);
                                }
                                ComponentTypeDeclaration::Alias(a) => {
                                    new_comp.alias(process_alias(a, &mut reencode));
                                }
                                ComponentTypeDeclaration::Export { name, ty } => {
                                    new_comp.export(name.0, reencode.component_type_ref(*ty));
                                }
                                ComponentTypeDeclaration::Import(imp) => {
                                    new_comp
                                        .import(imp.name.0, reencode.component_type_ref(imp.ty));
                                }
                            }
                        }
                        component_ty_section.component(&new_comp);
                    }
                    ComponentType::Instance(inst) => {
                        for i in inst.iter() {
                            component_ty_section
                                .instance(&convert_instance_type((*i).clone(), &mut reencode));
                        }
                    }
                    ComponentType::Resource { rep, dtor } => {
                        component_ty_section.resource(reencode.val_type(*rep).unwrap(), *dtor);
                    }
                }
            }
            component.section(&component_ty_section);
        }

        // Import parsing
        if !self.imports.is_empty() {
            let mut imports = wasm_encoder::ComponentImportSection::new();
            for imp in self.imports.iter() {
                imports.import(imp.name.0, reencode.component_type_ref(imp.ty));
            }
            component.section(&imports);
        }

        // Export parsing
        if !self.exports.is_empty() {
            let mut exports = wasm_encoder::ComponentExportSection::new();
            for exp in self.exports.iter() {
                exports.export(
                    exp.name.0,
                    reencode.component_export_kind(exp.kind),
                    exp.index,
                    exp.ty.map(|ty| reencode.component_type_ref(ty)),
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
                            args.iter().map(|arg| {
                                (
                                    arg.name,
                                    reencode.component_export_kind(arg.kind),
                                    arg.index,
                                )
                            }),
                        );
                    }
                    ComponentInstance::FromExports(export) => {
                        instances.export_items(export.iter().map(|value| {
                            (
                                value.name.0,
                                reencode.component_export_kind(value.kind),
                                value.index,
                            )
                        }));
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
                        instances.instantiate(
                            *module_index,
                            args.iter()
                                .map(|arg| (arg.name, ModuleArg::Instance(arg.index))),
                        );
                    }
                    Instance::FromExports(exports) => {
                        instances.export_items(exports.iter().map(|export| {
                            (
                                export.name,
                                wasm_encoder::ExportKind::from(export.kind),
                                export.index,
                            )
                        }));
                    }
                }
            }
            component.section(&instances);
        }

        // Alias parsing
        if !self.alias.is_empty() {
            let mut alias = ComponentAliasSection::new();
            for a in self.alias.iter() {
                alias.alias(process_alias(a, &mut reencode));
            }
            component.section(&alias);
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
                            options
                                .iter()
                                .map(|canon| reencode.canonical_option(*canon)),
                        );
                    }
                    CanonicalFunction::Lower {
                        func_index,
                        options,
                    } => {
                        canon_sec.lower(
                            *func_index,
                            options
                                .iter()
                                .map(|canon| reencode.canonical_option(*canon)),
                        );
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
        component.finish()
    }

    /// Print every instruction
    pub fn visitor(&self) {
        // This function is responsible for visiting every instruction
        for (index, module) in self.modules.iter().enumerate() {
            println!("Entered Module: {}", index);
            (*module).clone().visitor();
        }
    }

    // flip the instrument type according to the instruction of interest
    // pub fn mark_as_instrument(&mut self, interest_instr: Vec<(Operator, InstrumentType)>) {
    //     // This function is responsible for visiting every instruction
    //     for (mod_idx, module) in self.modules.iter_mut().enumerate() {
    //         println!("Entered Module: {}", mod_idx);
    //         for (func_idx, body) in module.code_sections.iter_mut().enumerate() {
    //             println!("Entered Function: {}", func_idx);
    //             // Each function index should match to a code section
    //             // for (local_idx, local_ty) in body.locals.iter() {
    //             //     println!("Local {}: {}", local_idx, local_ty);
    //             // }
    //             for (instr_idx, (instr, ref mut instrumented)) in
    //                 body.instructions.iter_mut().enumerate()
    //             {
    //                 println!(" {}: {:?}, {}", instr_idx, instr, instrumented);
    //                 *instrumented = compare_operator_instr_ty(interest_instr.clone(), instr);
    //             }
    //         }
    //     }
    // }

    // pub fn add_instrumentation(&mut self, code_injections: Vec<(Operator<'a>, Operator<'a>)>) {
    //     for (_module_idx, module) in self.modules.iter_mut().enumerate() {
    //         for (_fun_idx, body) in module.code_sections.iter_mut().enumerate() {
    //             // Each function index should match to a code section
    //             for (local_idx, local_ty) in body.locals.iter() {
    //                 println!("Local {}: {}", local_idx, local_ty);
    //             }
    //
    //             let mut changes = Vec::new();
    //
    //             for (idx, (instr, instrumented)) in body.instructions.iter_mut().enumerate() {
    //                 if *instrumented != InstrumentType::NotInstrumented {
    //                     match compare_operator_for_inject(code_injections.clone(), instr.clone()) {
    //                         Some(inject) => {
    //                             if *instrumented == InstrumentType::InstrumentAlternate {
    //                                 // Replace the value
    //                                 changes.push((idx, inject, InstrumentType::NotInstrumented));
    //                             } else if *instrumented == InstrumentType::InstrumentBefore {
    //                                 *instrumented = InstrumentType::NotInstrumented;
    //                                 changes.push((idx, inject, InstrumentType::NotInstrumented));
    //                             } else {
    //                                 *instrumented = InstrumentType::NotInstrumented;
    //                                 changes.push((
    //                                     idx + 1,
    //                                     inject,
    //                                     InstrumentType::NotInstrumented,
    //                                 ));
    //                             }
    //                         }
    //                         None => {
    //                             // If nothing matches, reset instrumentation
    //                             *instrumented = InstrumentType::NotInstrumented;
    //                         }
    //                     }
    //                 }
    //             }
    //
    //             // Apply changes
    //             let mut offset = 0;
    //             for (idx, op, instr) in changes {
    //                 if instr == InstrumentType::InstrumentAlternate {
    //                     body.instructions[idx + offset] = (op, instr);
    //                 } else {
    //                     body.instructions
    //                         .insert(idx + offset, (op, InstrumentType::NotInstrumented));
    //                     offset += 1;
    //                 }
    //             }
    //         }
    //     }
    // }
}