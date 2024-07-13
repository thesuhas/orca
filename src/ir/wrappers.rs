use wasm_encoder::reencode::Reencode;
use wasm_encoder::{
    Alias, ComponentFuncTypeEncoder, ComponentTypeEncoder, CoreTypeEncoder, InstanceType,
};
use wasmparser::{
    ComponentAlias, ComponentFuncResult, ComponentType, ComponentTypeDeclaration, CoreType,
    InstanceTypeDeclaration, SubType,
};

// Not added to wasm-tools
/// Convert ModuleTypeDeclaration to ModuleType
pub fn convert_module_type_declaration(
    ty: wasmparser::ModuleTypeDeclaration,
    reencode: &mut wasm_encoder::reencode::RoundtripReencoder,
) -> wasm_encoder::ModuleType {
    let mut mty = wasm_encoder::ModuleType::new();
    match ty {
        wasmparser::ModuleTypeDeclaration::Type(sub) => {
            let enc = mty.ty();
            encode_core_type_subtype(enc, &sub, reencode);
        }
        wasmparser::ModuleTypeDeclaration::Export { name, ty } => {
            mty.export(name, reencode.entity_type(ty).unwrap());
        }
        wasmparser::ModuleTypeDeclaration::OuterAlias {
            kind: _kind,
            count,
            index,
        } => {
            mty.alias_outer_core_type(count, index);
        }
        wasmparser::ModuleTypeDeclaration::Import(import) => {
            mty.import(
                import.module,
                import.name,
                reencode.entity_type(import.ty).unwrap(),
            );
        }
    }
    mty
}

// Not added to wasm-tools
/// Convert Instance Types
pub fn convert_instance_type(
    value: InstanceTypeDeclaration,
    reencode: &mut wasm_encoder::reencode::RoundtripReencoder,
) -> InstanceType {
    let mut ity = InstanceType::new();
    match value {
        InstanceTypeDeclaration::CoreType(core_type) => match core_type {
            wasmparser::CoreType::Sub(sub) => {
                let enc = ity.core_type();
                encode_core_type_subtype(enc, &sub, reencode);
            }
            wasmparser::CoreType::Module(module) => {
                for m in &*module {
                    let enc = ity.core_type();
                    enc.module(&convert_module_type_declaration((*m).clone(), reencode));
                }
            }
        },
        InstanceTypeDeclaration::Type(ty) => {
            let enc = ity.ty();
            convert_component_type(ty, enc, reencode);
        }
        InstanceTypeDeclaration::Alias(alias) => match alias {
            ComponentAlias::InstanceExport {
                kind,
                instance_index,
                name,
            } => {
                ity.alias(Alias::InstanceExport {
                    instance: instance_index,
                    kind: reencode.component_export_kind(kind),
                    name,
                });
            }
            ComponentAlias::CoreInstanceExport {
                kind,
                instance_index,
                name,
            } => {
                ity.alias(Alias::CoreInstanceExport {
                    instance: instance_index,
                    kind: reencode.export_kind(kind),
                    name,
                });
            }
            ComponentAlias::Outer { kind, count, index } => {
                ity.alias(Alias::Outer {
                    kind: reencode.component_outer_alias_kind(kind),
                    count,
                    index,
                });
            }
        },
        InstanceTypeDeclaration::Export { name, ty } => {
            ity.export(name.0, reencode.component_type_ref(ty));
        }
    }
    ity
}

// Not added to wasm-tools
/// Convert Func Results
pub fn convert_results(
    result: ComponentFuncResult,
    mut enc: ComponentFuncTypeEncoder,
    reencode: &mut wasm_encoder::reencode::RoundtripReencoder,
) {
    let mut results = vec![];
    match result {
        // Modified to pass encoder into this function, need to use result for unnamed - https://github.com/bytecodealliance/wasm-tools/discussions/1639#discussioncomment-9887694
        ComponentFuncResult::Unnamed(ty) => {
            enc.result(reencode.component_val_type(ty));
        }
        ComponentFuncResult::Named(b) => {
            for named in b.into_vec() {
                results.push((named.0, reencode.component_val_type(named.1)));
            }
        }
    }
    enc.results(results);
}

// Not added to wasm-tools
/// Convert variant case
pub fn convert_variant_case<'a>(
    variant: &'a wasmparser::VariantCase<'a>,
) -> (&'a str, Option<wasm_encoder::ComponentValType>, Option<u32>) {
    let mut reencode = wasm_encoder::reencode::RoundtripReencoder;
    (
        variant.name,
        variant.ty.map(|ty| reencode.component_val_type(ty)),
        variant.refines,
    )
}

// Not added to wasm-tools
/// CoreTypeEncoding
pub fn encode_core_type_subtype(
    enc: CoreTypeEncoder,
    subtype: &SubType,
    reencode: &mut wasm_encoder::reencode::RoundtripReencoder,
) {
    // TODO: Struct and Arrays once added to wasm_encoder - still in GC Proposal
    match &subtype.composite_type.inner {
        wasmparser::CompositeInnerType::Func(func) => {
            enc.function(
                func.params()
                    .iter()
                    .map(|val_type| reencode.val_type(*val_type).unwrap())
                    .collect::<Vec<_>>(),
                func.results()
                    .iter()
                    .map(|val_type| reencode.val_type(*val_type).unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        wasmparser::CompositeInnerType::Array(_) | wasmparser::CompositeInnerType::Struct(_) => {
            panic!("Still in GC Proposal")
        }
    }
}

/// Process Alias
pub fn process_alias<'a>(
    a: &'a ComponentAlias<'a>,
    reencode: &mut wasm_encoder::reencode::RoundtripReencoder,
) -> Alias<'a> {
    match a {
        ComponentAlias::InstanceExport {
            kind,
            instance_index,
            name,
        } => Alias::InstanceExport {
            instance: *instance_index,
            kind: reencode.component_export_kind(*kind),
            name,
        },
        ComponentAlias::CoreInstanceExport {
            kind,
            instance_index,
            name,
        } => Alias::CoreInstanceExport {
            instance: *instance_index,
            kind: reencode.export_kind(*kind),
            name,
        },
        ComponentAlias::Outer { kind, count, index } => Alias::Outer {
            kind: reencode.component_outer_alias_kind(*kind),
            count: *count,
            index: *index,
        },
    }
}

/// Convert Component Type
pub fn convert_component_type(
    ty: ComponentType,
    enc: ComponentTypeEncoder,
    reencode: &mut wasm_encoder::reencode::RoundtripReencoder,
) {
    match ty {
        ComponentType::Defined(comp_ty) => {
            let def_enc = enc.defined_type();
            match comp_ty {
                wasmparser::ComponentDefinedType::Primitive(p) => {
                    def_enc.primitive(wasm_encoder::PrimitiveValType::from(p))
                }
                wasmparser::ComponentDefinedType::Record(record) => {
                    def_enc.record(
                        record
                            .iter()
                            .map(|record| (record.0, reencode.component_val_type(record.1))),
                    );
                }
                wasmparser::ComponentDefinedType::Variant(variant) => {
                    def_enc.variant(variant.iter().map(convert_variant_case))
                }
                wasmparser::ComponentDefinedType::List(l) => {
                    def_enc.list(reencode.component_val_type(l))
                }
                wasmparser::ComponentDefinedType::Tuple(tup) => def_enc.tuple(
                    tup.into_vec()
                        .into_iter()
                        .map(|val_type| reencode.component_val_type(val_type)),
                ),
                wasmparser::ComponentDefinedType::Flags(flags) => def_enc.flags(flags.into_vec()),
                wasmparser::ComponentDefinedType::Enum(en) => def_enc.enum_type(en.into_vec()),
                wasmparser::ComponentDefinedType::Option(opt) => {
                    def_enc.option(reencode.component_val_type(opt))
                }
                wasmparser::ComponentDefinedType::Result { ok, err } => def_enc.result(
                    ok.map(|val_type| reencode.component_val_type(val_type)),
                    err.map(|val_type| reencode.component_val_type(val_type)),
                ),
                wasmparser::ComponentDefinedType::Own(u) => def_enc.own(u),
                wasmparser::ComponentDefinedType::Borrow(u) => def_enc.borrow(u),
            }
        }
        ComponentType::Func(func_ty) => {
            let mut new_enc = enc.function();
            new_enc.params(
                func_ty
                    .params
                    .into_vec()
                    .into_iter()
                    .map(|p| (p.0, reencode.component_val_type(p.1))),
            );
            convert_results(func_ty.results, new_enc, reencode);
        }
        ComponentType::Component(comp) => {
            let mut new_comp = wasm_encoder::ComponentType::new();
            for c in comp.into_vec().into_iter() {
                match c {
                    ComponentTypeDeclaration::CoreType(core) => match core {
                        CoreType::Sub(sub) => {
                            let enc = new_comp.core_type();
                            encode_core_type_subtype(enc, &sub, reencode);
                        }
                        CoreType::Module(module) => {
                            for m in &*module {
                                let enc = new_comp.core_type();
                                enc.module(&convert_module_type_declaration(
                                    (*m).clone(),
                                    reencode,
                                ));
                            }
                        }
                    },
                    ComponentTypeDeclaration::Type(typ) => {
                        let enc = new_comp.ty();
                        convert_component_type(typ, enc, reencode);
                    }
                    ComponentTypeDeclaration::Alias(a) => {
                        new_comp.alias(process_alias(&a, reencode));
                    }
                    ComponentTypeDeclaration::Export { name, ty } => {
                        new_comp.export(name.0, reencode.component_type_ref(ty));
                    }
                    ComponentTypeDeclaration::Import(imp) => {
                        new_comp.import(imp.name.0, reencode.component_type_ref(imp.ty));
                    }
                }
            }
        }
        ComponentType::Instance(inst) => {
            let mut ity = wasm_encoder::InstanceType::new();
            for i in inst.into_vec() {
                match i {
                    InstanceTypeDeclaration::CoreType(core_type) => match core_type {
                        wasmparser::CoreType::Sub(sub) => {
                            let enc = ity.core_type();
                            encode_core_type_subtype(enc, &sub, reencode);
                        }
                        wasmparser::CoreType::Module(module) => {
                            for m in &*module {
                                let enc = ity.core_type();
                                enc.module(&convert_module_type_declaration(
                                    (*m).clone(),
                                    reencode,
                                ));
                            }
                        }
                    },
                    InstanceTypeDeclaration::Type(ty) => {
                        let enc = ity.ty();
                        convert_component_type(ty, enc, reencode);
                    }
                    InstanceTypeDeclaration::Alias(alias) => match alias {
                        ComponentAlias::InstanceExport {
                            kind,
                            instance_index,
                            name,
                        } => {
                            ity.alias(Alias::InstanceExport {
                                instance: instance_index,
                                kind: reencode.component_export_kind(kind),
                                name,
                            });
                        }
                        ComponentAlias::CoreInstanceExport {
                            kind,
                            instance_index,
                            name,
                        } => {
                            ity.alias(Alias::CoreInstanceExport {
                                instance: instance_index,
                                kind: reencode.export_kind(kind),
                                name,
                            });
                        }
                        ComponentAlias::Outer { kind, count, index } => {
                            ity.alias(Alias::Outer {
                                kind: reencode.component_outer_alias_kind(kind),
                                count,
                                index,
                            });
                        }
                    },
                    InstanceTypeDeclaration::Export { name, ty } => {
                        ity.export(name.0, reencode.component_type_ref(ty));
                    }
                }
            }
            enc.instance(&ity);
        }
        ComponentType::Resource { rep, dtor } => {
            enc.resource(reencode.val_type(rep).unwrap(), dtor);
        }
    }
}
