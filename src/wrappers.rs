use wasm_encoder::{Alias, ComponentExportKind, ComponentFuncTypeEncoder, ComponentTypeEncoder, CoreTypeEncoder, EntityType, ExportKind, InstanceType, ModuleArg};
use wasmparser::{ComponentAlias, ComponentExternalKind, ComponentFuncResult, ComponentInstantiationArg, ComponentType, ComponentTypeDeclaration, CoreType, ExternalKind, InstanceTypeDeclaration, SubType, TypeRef};

/// Wrapper for Component External Kind to convert to wasm_encoder compatible enum
pub struct EncoderComponentExportKind(ComponentExportKind);

impl From<ComponentExternalKind> for EncoderComponentExportKind {
    fn from(value: ComponentExternalKind) -> Self {
        match value {
            ComponentExternalKind::Module => {
                EncoderComponentExportKind(ComponentExportKind::Module)
            }
            ComponentExternalKind::Func => EncoderComponentExportKind(ComponentExportKind::Func),
            ComponentExternalKind::Value => EncoderComponentExportKind(ComponentExportKind::Value),
            ComponentExternalKind::Type => EncoderComponentExportKind(ComponentExportKind::Type),
            ComponentExternalKind::Instance => {
                EncoderComponentExportKind(ComponentExportKind::Instance)
            }
            ComponentExternalKind::Component => {
                EncoderComponentExportKind(ComponentExportKind::Component)
            }
        }
    }
}

impl EncoderComponentExportKind {
    pub fn ret_original(&self) -> ComponentExportKind {
        match self {
            EncoderComponentExportKind(ComponentExportKind::Module) => ComponentExportKind::Module,
            EncoderComponentExportKind(ComponentExportKind::Func) => ComponentExportKind::Func,
            EncoderComponentExportKind(ComponentExportKind::Value) => ComponentExportKind::Value,
            EncoderComponentExportKind(ComponentExportKind::Type) => ComponentExportKind::Type,
            EncoderComponentExportKind(ComponentExportKind::Instance) => {
                ComponentExportKind::Instance
            }
            EncoderComponentExportKind(ComponentExportKind::Component) => {
                ComponentExportKind::Component
            }
        }
    }
}

/// Wrapper for External Kind to convert to wasm_encoder compatible enum
pub struct EncoderExportKind(ExportKind);

impl From<ExternalKind> for EncoderExportKind {
    fn from(value: ExternalKind) -> Self {
        match value {
            ExternalKind::Func => EncoderExportKind(ExportKind::Func),
            ExternalKind::Table => EncoderExportKind(ExportKind::Table),
            ExternalKind::Memory => EncoderExportKind(ExportKind::Memory),
            ExternalKind::Global => EncoderExportKind(ExportKind::Global),
            ExternalKind::Tag => EncoderExportKind(ExportKind::Tag),
        }
    }
}

impl EncoderExportKind {
    pub fn ret_original(&self) -> ExportKind {
        match self {
            EncoderExportKind(ExportKind::Func) => ExportKind::Func,
            EncoderExportKind(ExportKind::Table) => ExportKind::Table,
            EncoderExportKind(ExportKind::Memory) => ExportKind::Memory,
            EncoderExportKind(ExportKind::Global) => ExportKind::Global,
            EncoderExportKind(ExportKind::Tag) => ExportKind::Tag,
        }
    }
}

/// Wrapper for Component Outer Alias Kind to convert to wasm_encoder compatible enum
pub struct EncoderComponentOuterAlias(wasm_encoder::ComponentOuterAliasKind);

impl From<wasmparser::ComponentOuterAliasKind> for EncoderComponentOuterAlias {
    fn from(value: wasmparser::ComponentOuterAliasKind) -> Self {
        match value {
            wasmparser::ComponentOuterAliasKind::CoreModule => {
                EncoderComponentOuterAlias(wasm_encoder::ComponentOuterAliasKind::CoreModule)
            }
            wasmparser::ComponentOuterAliasKind::CoreType => {
                EncoderComponentOuterAlias(wasm_encoder::ComponentOuterAliasKind::CoreType)
            }
            wasmparser::ComponentOuterAliasKind::Type => {
                EncoderComponentOuterAlias(wasm_encoder::ComponentOuterAliasKind::Type)
            }
            wasmparser::ComponentOuterAliasKind::Component => {
                EncoderComponentOuterAlias(wasm_encoder::ComponentOuterAliasKind::Component)
            }
        }
    }
}

impl EncoderComponentOuterAlias {
    pub fn ret_original(&self) -> wasm_encoder::ComponentOuterAliasKind {
        match self {
            EncoderComponentOuterAlias(wasm_encoder::ComponentOuterAliasKind::CoreModule) => {
                wasm_encoder::ComponentOuterAliasKind::CoreModule
            }
            EncoderComponentOuterAlias(wasm_encoder::ComponentOuterAliasKind::CoreType) => {
                wasm_encoder::ComponentOuterAliasKind::CoreType
            }
            EncoderComponentOuterAlias(wasm_encoder::ComponentOuterAliasKind::Type) => {
                wasm_encoder::ComponentOuterAliasKind::Type
            }
            EncoderComponentOuterAlias(wasm_encoder::ComponentOuterAliasKind::Component) => {
                wasm_encoder::ComponentOuterAliasKind::Component
            }
        }
    }
}

/// Wrapper around ComponentTypeRef
pub struct EncoderComponentTypeRef(wasm_encoder::ComponentTypeRef);

impl From<wasmparser::ComponentTypeRef> for EncoderComponentTypeRef {
    fn from(value: wasmparser::ComponentTypeRef) -> Self {
        match value {
            wasmparser::ComponentTypeRef::Module(u) => {
                EncoderComponentTypeRef(wasm_encoder::ComponentTypeRef::Module(u))
            }
            wasmparser::ComponentTypeRef::Func(u) => {
                EncoderComponentTypeRef(wasm_encoder::ComponentTypeRef::Func(u))
            }
            wasmparser::ComponentTypeRef::Value(ty) => {
                EncoderComponentTypeRef(wasm_encoder::ComponentTypeRef::Value(
                    EncoderComponentValType::from(ty).ret_original(),
                ))
            }
            wasmparser::ComponentTypeRef::Type(ty) => EncoderComponentTypeRef(
                wasm_encoder::ComponentTypeRef::Type(EncoderTypeBounds::from(ty).ret_original()),
            ),
            wasmparser::ComponentTypeRef::Instance(u) => {
                EncoderComponentTypeRef(wasm_encoder::ComponentTypeRef::Instance(u))
            }
            wasmparser::ComponentTypeRef::Component(u) => {
                EncoderComponentTypeRef(wasm_encoder::ComponentTypeRef::Component(u))
            }
        }
    }
}

impl EncoderComponentTypeRef {
    pub fn ret_original(&self) -> wasm_encoder::ComponentTypeRef {
        match self {
            EncoderComponentTypeRef(wasm_encoder::ComponentTypeRef::Module(u)) => {
                wasm_encoder::ComponentTypeRef::Module(*u)
            }
            EncoderComponentTypeRef(wasm_encoder::ComponentTypeRef::Func(u)) => {
                wasm_encoder::ComponentTypeRef::Func(*u)
            }
            EncoderComponentTypeRef(wasm_encoder::ComponentTypeRef::Value(ty)) => {
                wasm_encoder::ComponentTypeRef::Value(*ty)
            }
            EncoderComponentTypeRef(wasm_encoder::ComponentTypeRef::Type(ty)) => {
                wasm_encoder::ComponentTypeRef::Type(*ty)
            }
            EncoderComponentTypeRef(wasm_encoder::ComponentTypeRef::Instance(u)) => {
                wasm_encoder::ComponentTypeRef::Instance(*u)
            }
            EncoderComponentTypeRef(wasm_encoder::ComponentTypeRef::Component(u)) => {
                wasm_encoder::ComponentTypeRef::Component(*u)
            }
        }
    }
}

/// Wrapper around TypeBounds
pub struct EncoderTypeBounds(wasm_encoder::TypeBounds);

impl From<wasmparser::TypeBounds> for EncoderTypeBounds {
    fn from(value: wasmparser::TypeBounds) -> Self {
        match value {
            wasmparser::TypeBounds::Eq(u) => EncoderTypeBounds(wasm_encoder::TypeBounds::Eq(u)),
            wasmparser::TypeBounds::SubResource => {
                EncoderTypeBounds(wasm_encoder::TypeBounds::SubResource)
            }
        }
    }
}

impl EncoderTypeBounds {
    pub fn ret_original(&self) -> wasm_encoder::TypeBounds {
        match self {
            EncoderTypeBounds(wasm_encoder::TypeBounds::Eq(u)) => wasm_encoder::TypeBounds::Eq(*u),
            EncoderTypeBounds(wasm_encoder::TypeBounds::SubResource) => {
                wasm_encoder::TypeBounds::SubResource
            }
        }
    }
}

/// Wrapper around ComponentValType
pub struct EncoderComponentValType(wasm_encoder::ComponentValType);

impl From<wasmparser::ComponentValType> for EncoderComponentValType {
    fn from(value: wasmparser::ComponentValType) -> Self {
        match value {
            wasmparser::ComponentValType::Type(u) => {
                EncoderComponentValType(wasm_encoder::ComponentValType::Type(u))
            }
            wasmparser::ComponentValType::Primitive(ty) => EncoderComponentValType(
                wasm_encoder::ComponentValType::Primitive(wasm_encoder::PrimitiveValType::from(ty)),
            ),
        }
    }
}

impl EncoderComponentValType {
    pub fn ret_original(&self) -> wasm_encoder::ComponentValType {
        match self {
            EncoderComponentValType(wasm_encoder::ComponentValType::Type(u)) => {
                wasm_encoder::ComponentValType::Type(*u)
            }
            EncoderComponentValType(wasm_encoder::ComponentValType::Primitive(ty)) => {
                wasm_encoder::ComponentValType::Primitive(*ty)
            }
        }
    }
}

pub fn convert_val_type(ty: &wasmparser::ValType) -> wasm_encoder::ValType {
    EncoderValType::from(*ty).ret_original()
}

/// Function that converts Canonical Option
pub fn convert_canon(value: wasmparser::CanonicalOption) -> wasm_encoder::CanonicalOption {
    match value {
        wasmparser::CanonicalOption::UTF8 => wasm_encoder::CanonicalOption::UTF8,
        wasmparser::CanonicalOption::UTF16 => wasm_encoder::CanonicalOption::UTF16,
        wasmparser::CanonicalOption::CompactUTF16 => wasm_encoder::CanonicalOption::CompactUTF16,
        wasmparser::CanonicalOption::Memory(u) => wasm_encoder::CanonicalOption::Memory(u),
        wasmparser::CanonicalOption::Realloc(u) => wasm_encoder::CanonicalOption::Realloc(u),
        wasmparser::CanonicalOption::PostReturn(u) => wasm_encoder::CanonicalOption::PostReturn(u),
    }
}

/// Extracts the args of ComponentInstantiation
pub fn convert_component_instantiation_arg(
    arg: ComponentInstantiationArg,
) -> (&str, ComponentExportKind, u32) {
    (
        arg.name,
        EncoderComponentExportKind::from(arg.kind).ret_original(),
        arg.index,
    )
}

/// Extracts and Converts Component Export
pub fn convert_component_export(
    value: wasmparser::ComponentExport,
) -> (&str, ComponentExportKind, u32) {
    (
        value.name.0,
        EncoderComponentExportKind::from(value.kind).ret_original(),
        value.index,
    )
}

/// Extracts and Converts Exports
pub fn convert_export(value: wasmparser::Export) -> (&str, ExportKind, u32) {
    (
        value.name,
        wasm_encoder::ExportKind::from(value.kind),
        value.index,
    )
}

/// Extracts and Converts Instance args
pub fn convert_instantiation_arg(value: wasmparser::InstantiationArg) -> (&str, ModuleArg) {
    (value.name, ModuleArg::Instance(value.index))
}

/// Convert wasmparser TypeRef to Entity Type
pub struct EncoderEntityType(EntityType);

impl From<TypeRef> for EncoderEntityType {
    fn from(value: TypeRef) -> Self {
        match value {
            TypeRef::Func(u) => EncoderEntityType(EntityType::Function(u)),
            TypeRef::Table(ty) => EncoderEntityType(EntityType::Table(
                wasm_encoder::TableType::try_from(ty).unwrap(),
            )),
            TypeRef::Memory(ty) => {
                EncoderEntityType(EntityType::Memory(wasm_encoder::MemoryType::from(ty)))
            }
            TypeRef::Global(ty) => EncoderEntityType(EntityType::Global(
                wasm_encoder::GlobalType::try_from(ty).unwrap(),
            )),
            TypeRef::Tag(ty) => EncoderEntityType(EntityType::Tag(
                wasm_encoder::TagType::try_from(ty).unwrap(),
            )),
        }
    }
}

impl EncoderEntityType {
    pub fn ret_original(&self) -> EntityType {
        match self {
            EncoderEntityType(EntityType::Function(u)) => EntityType::Function(*u),
            EncoderEntityType(EntityType::Table(ty)) => EntityType::Table(*ty),
            EncoderEntityType(EntityType::Global(ty)) => EntityType::Global(*ty),
            EncoderEntityType(EntityType::Memory(ty)) => EntityType::Memory(*ty),
            EncoderEntityType(EntityType::Tag(ty)) => EntityType::Tag(*ty),
        }
    }
}

/// Convert ModuleTypeDeclaration to ModuleType
pub fn convert_module_type_declaration(
    ty: wasmparser::ModuleTypeDeclaration,
) -> wasm_encoder::ModuleType {
    let mut mty = wasm_encoder::ModuleType::new();
    match ty {
        wasmparser::ModuleTypeDeclaration::Type(sub) => {
            let enc = mty.ty();
            encode_core_type_subtype(enc, sub);
        }
        wasmparser::ModuleTypeDeclaration::Export { name, ty } => {
            mty.export(name, EncoderEntityType::from(ty).ret_original());
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
                EncoderEntityType::from(import.ty).ret_original(),
            );
        }
    }
    return mty;
}

/// Convert Instance Types
pub fn convert_instance_type(value: InstanceTypeDeclaration) -> InstanceType {
    let mut ity = InstanceType::new();
    match value {
        InstanceTypeDeclaration::CoreType(core_type) => match core_type {
            wasmparser::CoreType::Sub(sub) => {
                let enc = ity.core_type();
                encode_core_type_subtype(enc, sub);
            }
            wasmparser::CoreType::Module(module) => {
                for m in &*module {
                    let enc = ity.core_type();
                    enc.module(&convert_module_type_declaration((*m).clone()));
                }
            }
        },
        InstanceTypeDeclaration::Type(ty) => {
            let enc = ity.ty();
            convert_component_type(ty, enc);
        }
        InstanceTypeDeclaration::Alias(alias) => match alias {
            ComponentAlias::InstanceExport {
                kind,
                instance_index,
                name,
            } => {
                ity.alias(Alias::InstanceExport {
                    instance: instance_index,
                    kind: EncoderComponentExportKind::from(kind).ret_original(),
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
                    kind: EncoderExportKind::from(kind).ret_original(),
                    name,
                });
            }
            ComponentAlias::Outer { kind, count, index } => {
                ity.alias(Alias::Outer {
                    kind: EncoderComponentOuterAlias::from(kind).ret_original(),
                    count,
                    index,
                });
            }
        },
        InstanceTypeDeclaration::Export { name, ty } => {
            ity.export(name.0, EncoderComponentTypeRef::from(ty).ret_original());
        }
    }
    return ity;
}

/// Wrapper for HeapType Conversion

pub fn convert_heap_type(value: wasmparser::HeapType) -> wasm_encoder::HeapType {
    match value {
        wasmparser::HeapType::Concrete(u) => {
            // Panic is caused because of - https://github.com/bytecodealliance/wasm-tools/discussions/1639#discussioncomment-9887694
            match u {
                wasmparser::UnpackedIndex::Module(u) => wasm_encoder::HeapType::Concrete(u),
                wasmparser::UnpackedIndex::RecGroup(_) | wasmparser::UnpackedIndex::Id(_) => {
                    panic!("Only for validation")
                }
            }
        }
        wasmparser::HeapType::Abstract { shared, ty } => wasm_encoder::HeapType::Abstract {
            shared,
            ty: match ty {
                wasmparser::AbstractHeapType::Func => wasm_encoder::AbstractHeapType::Func,
                wasmparser::AbstractHeapType::Extern => wasm_encoder::AbstractHeapType::Extern,
                wasmparser::AbstractHeapType::Any => wasm_encoder::AbstractHeapType::Any,
                wasmparser::AbstractHeapType::None => wasm_encoder::AbstractHeapType::None,
                wasmparser::AbstractHeapType::NoExtern => wasm_encoder::AbstractHeapType::None,
                wasmparser::AbstractHeapType::NoFunc => wasm_encoder::AbstractHeapType::NoFunc,
                wasmparser::AbstractHeapType::Eq => wasm_encoder::AbstractHeapType::Eq,
                wasmparser::AbstractHeapType::Struct => wasm_encoder::AbstractHeapType::Struct,
                wasmparser::AbstractHeapType::Array => wasm_encoder::AbstractHeapType::Array,
                wasmparser::AbstractHeapType::I31 => wasm_encoder::AbstractHeapType::I31,
                wasmparser::AbstractHeapType::Exn => wasm_encoder::AbstractHeapType::Exn,
                wasmparser::AbstractHeapType::NoExn => wasm_encoder::AbstractHeapType::NoExn,
            },
        },
    }
}

/// Wrapper for ValType Conversion
pub struct EncoderValType(wasm_encoder::ValType);

impl From<wasmparser::ValType> for EncoderValType {
    fn from(value: wasmparser::ValType) -> Self {
        match value {
            wasmparser::ValType::I32 => EncoderValType(wasm_encoder::ValType::I32),
            wasmparser::ValType::I64 => EncoderValType(wasm_encoder::ValType::I64),
            wasmparser::ValType::F32 => EncoderValType(wasm_encoder::ValType::F32),
            wasmparser::ValType::F64 => EncoderValType(wasm_encoder::ValType::F64),
            wasmparser::ValType::V128 => EncoderValType(wasm_encoder::ValType::V128),
            wasmparser::ValType::Ref(ty) => {
                EncoderValType(wasm_encoder::ValType::Ref(wasm_encoder::RefType {
                    nullable: ty.is_nullable(),
                    heap_type: convert_heap_type(ty.heap_type()),
                }))
            }
        }
    }
}

impl EncoderValType {
    pub fn ret_original(&self) -> wasm_encoder::ValType {
        match self {
            EncoderValType(wasm_encoder::ValType::I32) => wasm_encoder::ValType::I32,
            EncoderValType(wasm_encoder::ValType::I64) => wasm_encoder::ValType::I64,
            EncoderValType(wasm_encoder::ValType::F32) => wasm_encoder::ValType::F32,
            EncoderValType(wasm_encoder::ValType::F64) => wasm_encoder::ValType::F64,
            EncoderValType(wasm_encoder::ValType::V128) => wasm_encoder::ValType::V128,
            EncoderValType(wasm_encoder::ValType::Ref(ty)) => wasm_encoder::ValType::Ref(*ty),
        }
    }
}

/// Convert Component Val Type
pub fn convert_component_val_type(
    val: wasmparser::ComponentValType,
) -> wasm_encoder::ComponentValType {
    EncoderComponentValType::from(val).ret_original()
}

/// Convert Record Type
pub fn convert_record_type(
    val: (&str, wasmparser::ComponentValType),
) -> (&str, wasm_encoder::ComponentValType) {
    (val.0, EncoderComponentValType::from(val.1).ret_original())
}

/// Convert Func Params
pub fn convert_params(
    p: (&str, wasmparser::ComponentValType),
) -> (&str, wasm_encoder::ComponentValType) {
    (p.0, convert_component_val_type(p.1))
}

/// Convert Func Results
pub fn convert_results(result: ComponentFuncResult, mut enc: ComponentFuncTypeEncoder) {
    let mut results = vec![];
    match result {
        // Modified to pass encoder into this function, need to use result for unnamed - https://github.com/bytecodealliance/wasm-tools/discussions/1639#discussioncomment-9887694
        ComponentFuncResult::Unnamed(ty) => {
            enc.result(convert_component_val_type(ty));
        }
        ComponentFuncResult::Named(b) => {
            for named in b.into_vec() {
                results.push((named.0, convert_component_val_type(named.1)));
            }
        }
    }
    enc.results(results);
}

/// Convert variant case
pub fn convert_variant_case(
    variant: wasmparser::VariantCase,
) -> (&str, Option<wasm_encoder::ComponentValType>, Option<u32>) {
    (
        variant.name,
        match variant.ty {
            None => None,
            Some(ty) => Some(EncoderComponentValType::from(ty).ret_original()),
        },
        match variant.refines {
            None => None,
            Some(u) => Some(u),
        },
    )
}

/// CoreTypeEncoding
pub fn encode_core_type_subtype(enc: CoreTypeEncoder, subtype: SubType) {
    // TODO: Struct and Arrays once added to wasm_encoder - still in GC Proposal
    match subtype.composite_type {
        wasmparser::CompositeType::Func(func) => {
            enc.function(
                func.params()
                    .iter()
                    .map(convert_val_type)
                    .collect::<Vec<_>>(),
                func.results()
                    .iter()
                    .map(convert_val_type)
                    .collect::<Vec<_>>(),
            );
        }
        wasmparser::CompositeType::Array(_) | wasmparser::CompositeType::Struct(_) => {
            panic!("Still in GC Proposal")
        }
    }
}

/// Process Alias
pub fn process_alias(a: ComponentAlias) -> Alias {
    match a {
        ComponentAlias::InstanceExport {
            kind,
            instance_index,
            name,
        } => Alias::InstanceExport {
            instance: instance_index,
            kind: EncoderComponentExportKind::from(kind).ret_original(),
            name,
        },
        ComponentAlias::CoreInstanceExport {
            kind,
            instance_index,
            name,
        } => Alias::CoreInstanceExport {
            instance: instance_index,
            kind: EncoderExportKind::from(kind).ret_original(),
            name,
        },
        ComponentAlias::Outer { kind, count, index } => Alias::Outer {
            kind: EncoderComponentOuterAlias::from(kind).ret_original(),
            count,
            index,
        },
    }
}

/// Convert Component Type
pub fn convert_component_type(ty: ComponentType, enc: ComponentTypeEncoder) {
    match ty {
        ComponentType::Defined(comp_ty) => {
            let def_enc = enc.defined_type();
            match comp_ty {
                wasmparser::ComponentDefinedType::Primitive(p) => {
                    def_enc.primitive(wasm_encoder::PrimitiveValType::from(p))
                }
                wasmparser::ComponentDefinedType::Record(record) => {
                    def_enc.record(record.into_vec().into_iter().map(convert_record_type));
                }
                wasmparser::ComponentDefinedType::Variant(variant) => def_enc
                    .variant(variant.into_vec().into_iter().map(convert_variant_case)),
                wasmparser::ComponentDefinedType::List(l) => {
                    def_enc.list(EncoderComponentValType::from(l).ret_original())
                }
                wasmparser::ComponentDefinedType::Tuple(tup) => def_enc
                    .tuple(tup.into_vec().into_iter().map(convert_component_val_type)),
                wasmparser::ComponentDefinedType::Flags(flags) => {
                    def_enc.flags(flags.into_vec().into_iter())
                }
                wasmparser::ComponentDefinedType::Enum(en) => {
                    def_enc.enum_type(en.into_vec().into_iter())
                }
                wasmparser::ComponentDefinedType::Option(opt) => {
                    def_enc.option(convert_component_val_type(opt))
                }
                wasmparser::ComponentDefinedType::Result { ok, err } => def_enc.result(
                    match ok {
                        None => None,
                        Some(val) => Some(convert_component_val_type(val)),
                    },
                    match err {
                        None => None,
                        Some(e) => Some(convert_component_val_type(e)),
                    },
                ),
                wasmparser::ComponentDefinedType::Own(u) => def_enc.own(u),
                wasmparser::ComponentDefinedType::Borrow(u) => def_enc.borrow(u),
            }
        }
        ComponentType::Func(func_ty) => {
            let mut new_enc = enc.function();
            new_enc.params(func_ty.params.into_vec().into_iter().map(convert_params));
            convert_results(func_ty.results, new_enc);
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
        }
        ComponentType::Instance(_inst) => {
            // for i in inst.into_vec() {
            //     enc.instance(&convert_instance_type(i));
            // }
        }
        ComponentType::Resource { rep, dtor } => {
            enc.resource(EncoderValType::from(rep).ret_original(), dtor);
        }
    }
}