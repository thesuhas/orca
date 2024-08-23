//! Wrapper functions

use gimli::SectionId;
use std::collections::HashMap;
use wasm_encoder::reencode::Reencode;
use wasm_encoder::{
    Alias, ComponentFuncTypeEncoder, ComponentTypeEncoder, CoreTypeEncoder, InstanceType,
};
use wasmparser::{
    ComponentAlias, ComponentFuncResult, ComponentType, ComponentTypeDeclaration, CoreType,
    InstanceTypeDeclaration, Operator, SubType,
};

// Not added to wasm-tools
/// Convert ModuleTypeDeclaration to ModuleType
pub fn convert_module_type_declaration(
    module: &[wasmparser::ModuleTypeDeclaration],
    enc: CoreTypeEncoder,
    reencode: &mut wasm_encoder::reencode::RoundtripReencoder,
) {
    let mut mty = wasm_encoder::ModuleType::new();
    for m in module.iter() {
        match m {
            wasmparser::ModuleTypeDeclaration::Type(sub) => {
                let enc_mty = mty.ty();
                encode_core_type_subtype(enc_mty, sub, reencode);
            }
            wasmparser::ModuleTypeDeclaration::Export { name, ty } => {
                mty.export(name, reencode.entity_type(*ty).unwrap());
            }
            wasmparser::ModuleTypeDeclaration::OuterAlias {
                kind: _kind,
                count,
                index,
            } => {
                mty.alias_outer_core_type(*count, *index);
            }
            wasmparser::ModuleTypeDeclaration::Import(import) => {
                mty.import(
                    import.module,
                    import.name,
                    reencode.entity_type(import.ty).unwrap(),
                );
            }
        }
    }
    enc.module(&mty);
}

// Not added to wasm-tools
/// Convert Instance Types
pub fn convert_instance_type(
    instance: &[InstanceTypeDeclaration],
    reencode: &mut wasm_encoder::reencode::RoundtripReencoder,
) -> InstanceType {
    let mut ity = InstanceType::new();
    for value in instance.iter() {
        match value {
            InstanceTypeDeclaration::CoreType(core_type) => match core_type {
                CoreType::Sub(sub) => {
                    let enc = ity.core_type();
                    encode_core_type_subtype(enc, sub, reencode);
                }
                CoreType::Module(module) => {
                    let enc = ity.core_type();
                    convert_module_type_declaration(module, enc, reencode);
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
                        instance: *instance_index,
                        kind: reencode.component_export_kind(*kind),
                        name,
                    });
                }
                ComponentAlias::CoreInstanceExport {
                    kind,
                    instance_index,
                    name,
                } => {
                    ity.alias(Alias::CoreInstanceExport {
                        instance: *instance_index,
                        kind: reencode.export_kind(*kind),
                        name,
                    });
                }
                ComponentAlias::Outer { kind, count, index } => {
                    ity.alias(Alias::Outer {
                        kind: reencode.component_outer_alias_kind(*kind),
                        count: *count,
                        index: *index,
                    });
                }
            },
            InstanceTypeDeclaration::Export { name, ty } => {
                ity.export(name.0, reencode.component_type_ref(*ty));
            }
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
    match result {
        // Modified to pass encoder into this function, need to use result for unnamed - https://github.com/bytecodealliance/wasm-tools/discussions/1639#discussioncomment-9887694
        ComponentFuncResult::Unnamed(ty) => {
            enc.result(reencode.component_val_type(ty));
        }
        ComponentFuncResult::Named(b) => {
            let mut results = vec![];
            for named in b.into_vec() {
                results.push((named.0, reencode.component_val_type(named.1)));
            }
            enc.results(results);
        }
    }
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
    ty: &ComponentType,
    enc: ComponentTypeEncoder,
    reencode: &mut wasm_encoder::reencode::RoundtripReencoder,
) {
    match ty {
        ComponentType::Defined(comp_ty) => {
            let def_enc = enc.defined_type();
            match comp_ty {
                wasmparser::ComponentDefinedType::Primitive(p) => {
                    def_enc.primitive(wasm_encoder::PrimitiveValType::from(*p))
                }
                wasmparser::ComponentDefinedType::Record(record) => {
                    def_enc.record(
                        record
                            .iter()
                            .map(|record| (record.0, reencode.component_val_type(record.1))),
                    );
                }
                wasmparser::ComponentDefinedType::Variant(variant) => {
                    def_enc.variant(variant.iter().map(|variant| {
                        (
                            variant.name,
                            variant.ty.map(|ty| reencode.component_val_type(ty)),
                            variant.refines,
                        )
                    }))
                }
                wasmparser::ComponentDefinedType::List(l) => {
                    def_enc.list(reencode.component_val_type(*l))
                }
                wasmparser::ComponentDefinedType::Tuple(tup) => def_enc.tuple(
                    tup.iter()
                        .map(|val_type| reencode.component_val_type(*val_type)),
                ),
                wasmparser::ComponentDefinedType::Flags(flags) => {
                    def_enc.flags((*flags).clone().into_vec())
                }
                wasmparser::ComponentDefinedType::Enum(en) => {
                    def_enc.enum_type((*en).clone().into_vec())
                }
                wasmparser::ComponentDefinedType::Option(opt) => {
                    def_enc.option(reencode.component_val_type(*opt))
                }
                wasmparser::ComponentDefinedType::Result { ok, err } => def_enc.result(
                    ok.map(|val_type| reencode.component_val_type(val_type)),
                    err.map(|val_type| reencode.component_val_type(val_type)),
                ),
                wasmparser::ComponentDefinedType::Own(u) => def_enc.own(*u),
                wasmparser::ComponentDefinedType::Borrow(u) => def_enc.borrow(*u),
            }
        }
        ComponentType::Func(func_ty) => {
            let mut new_enc = enc.function();
            new_enc.params(
                func_ty
                    .clone()
                    .params
                    .into_vec()
                    .into_iter()
                    .map(|p| (p.0, reencode.component_val_type(p.1))),
            );
            convert_results(func_ty.clone().results, new_enc, reencode);
        }
        ComponentType::Component(comp) => {
            let mut new_comp = wasm_encoder::ComponentType::new();
            for c in comp.iter() {
                match c {
                    ComponentTypeDeclaration::CoreType(core) => match core {
                        CoreType::Sub(sub) => {
                            let enc = new_comp.core_type();
                            encode_core_type_subtype(enc, sub, reencode);
                        }
                        CoreType::Module(module) => {
                            let enc = new_comp.core_type();
                            convert_module_type_declaration(module, enc, reencode);
                        }
                    },
                    ComponentTypeDeclaration::Type(typ) => {
                        let enc = new_comp.ty();
                        convert_component_type(typ, enc, reencode);
                    }
                    ComponentTypeDeclaration::Alias(a) => {
                        new_comp.alias(process_alias(a, reencode));
                    }
                    ComponentTypeDeclaration::Export { name, ty } => {
                        new_comp.export(name.0, reencode.component_type_ref(*ty));
                    }
                    ComponentTypeDeclaration::Import(imp) => {
                        new_comp.import(imp.name.0, reencode.component_type_ref(imp.ty));
                    }
                }
            }
            enc.component(&new_comp);
        }
        ComponentType::Instance(inst) => {
            let ity = convert_instance_type(inst, reencode);
            enc.instance(&ity);
        }
        ComponentType::Resource { rep, dtor } => {
            enc.resource(reencode.val_type(*rep).unwrap(), *dtor);
        }
    }
}

pub fn indirect_namemap_parser2encoder(
    namemap: wasmparser::IndirectNameMap,
) -> wasm_encoder::IndirectNameMap {
    let mut names = wasm_encoder::IndirectNameMap::new();
    for name in namemap {
        let naming = name.unwrap();
        names.append(naming.index, &namemap_parser2encoder(naming.names));
    }
    names
}

pub fn namemap_parser2encoder(namemap: wasmparser::NameMap) -> wasm_encoder::NameMap {
    let mut names = wasm_encoder::NameMap::new();
    for name in namemap {
        let naming = name.unwrap();
        names.append(naming.index, naming.name);
    }
    names
}

pub fn add_to_namemap(namemap: &mut wasm_encoder::NameMap, names: wasmparser::NameMap) {
    for name in names {
        let naming = name.unwrap();
        namemap.append(naming.index, naming.name);
    }
}

pub fn get_section_id(name: &str) -> Option<SectionId> {
    match name {
        ".debug_abbrev" => Some(SectionId::DebugAbbrev),
        ".debug_addr" => Some(SectionId::DebugAddr),
        ".debug_aranges" => Some(SectionId::DebugAranges),
        ".debug_frame" => Some(SectionId::DebugFrame),
        ".debug_info" => Some(SectionId::DebugInfo),
        ".debug_line" => Some(SectionId::DebugLine),
        ".debug_line_str" => Some(SectionId::DebugLineStr),
        ".debug_loc" => Some(SectionId::DebugLoc),
        ".debug_loclists" => Some(SectionId::DebugLocLists),
        ".debug_pubnames" => Some(SectionId::DebugPubNames),
        ".debug_pubtypes" => Some(SectionId::DebugPubTypes),
        ".debug_ranges" => Some(SectionId::DebugRanges),
        ".debug_rnglists" => Some(SectionId::DebugRngLists),
        ".debug_str" => Some(SectionId::DebugStr),
        ".debug_str_offsets" => Some(SectionId::DebugStrOffsets),
        ".debug_types" => Some(SectionId::DebugTypes),
        ".debug_cu_index" => Some(SectionId::DebugCuIndex),
        ".debug_tu_index" => Some(SectionId::DebugTuIndex),
        ".eh_frame" => Some(SectionId::EhFrame),
        ".eh_frame_hdr" => Some(SectionId::EhFrameHdr),
        _ => None,
    }
}

pub(crate) fn refers_to_func(op: &Operator) -> bool {
    matches!(op, Operator::Call { .. } | Operator::RefFunc { .. })
}

pub(crate) fn update_fn_instr(op: &mut Operator, mapping: &HashMap<i32, i32>) {
    match op {
        Operator::Call { function_index } | Operator::RefFunc { function_index } => {
            match mapping.get(&(*function_index as i32)) {
                Some(new_index) => {
                    *function_index = *new_index as u32;
                }
                None => panic!("Deleted function!"),
            }
        }
        _ => panic!("Not a call operation!"),
    }
}
