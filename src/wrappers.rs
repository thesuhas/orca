use wasm_encoder::{Alias, ComponentExportKind, EntityType, ExportKind, InstanceType, ModuleArg};
use wasmparser::types::TypeIdentifier;
use wasmparser::{
    ComponentAlias, ComponentExternalKind, ComponentInstantiationArg, ExternalKind,
    InstanceTypeDeclaration, TypeRef,
};

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
        wasmparser::ModuleTypeDeclaration::Type(sub) => {}
        wasmparser::ModuleTypeDeclaration::Export { name, ty } => {
            mty.export(name, EncoderEntityType::from(ty).ret_original());
        }
        wasmparser::ModuleTypeDeclaration::OuterAlias { kind, count, index } => {
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

/// Convert Component Types
// pub fn convert_component_type(value: ComponentType) -> Vec<wasm_encoder::ComponentType> {
//     let mut types = vec![];
//     match value {
//         ComponentType::Defined(comp_ty) => {}
//         ComponentType::Func(func_ty) => {}
//         ComponentType::Component(comp) => {}
//         ComponentType::Instance(inst) => {
//             for i in inst.into_vec().into_iter() {
//                 types.push(&convert_instance_type(i));
//                 wasm_encoder::ComponentType::
//             }
//         }
//         ComponentType::Resource { rep, dtor } => {
//
//         }
//     }
//     return types;
// }

/// Convert Instance Types
pub fn convert_instance_type(value: InstanceTypeDeclaration) -> InstanceType {
    let mut ity = InstanceType::new();
    match value {
        InstanceTypeDeclaration::CoreType(core_type) => {
            match core_type {
                wasmparser::CoreType::Sub(sub) => {
                    // TODO: Struct and Arrays once added to wasm_encoder
                    let mut enc = ity.core_type();
                    match sub {
                        wasmparser::CompositeType::Func(func) => {
                            enc.function(func.params(), func.results());
                        }
                        wasmparser::CompositeType::Array(array) => {}
                        wasmparser::CompositeType::Struct(str) => {}
                    }
                }
                wasmparser::CoreType::Module(module) => {
                    for m in &*module {
                        let mut enc = ity.core_type();
                        enc.module(&convert_module_type_declaration((*m).clone()));
                    }
                }
            }
        }
        InstanceTypeDeclaration::Type(ty) => {
            // TODO - Recursive Data Structure
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
            // TODO - Check the conversion of indices
            match u {
                wasmparser::UnpackedIndex::Module(u) => wasm_encoder::HeapType::Concrete(u),
                wasmparser::UnpackedIndex::RecGroup(u) => wasm_encoder::HeapType::Concrete(u),
                wasmparser::UnpackedIndex::Id(id) => {
                    wasm_encoder::HeapType::Concrete(id.index() as u32)
                }
            }
        }
        wasmparser::HeapType::Func => wasm_encoder::HeapType::Func,
        wasmparser::HeapType::Extern => wasm_encoder::HeapType::Extern,
        wasmparser::HeapType::Any => wasm_encoder::HeapType::Any,
        wasmparser::HeapType::None => wasm_encoder::HeapType::None,
        wasmparser::HeapType::NoExtern => wasm_encoder::HeapType::None,
        wasmparser::HeapType::NoFunc => wasm_encoder::HeapType::NoFunc,
        wasmparser::HeapType::Eq => wasm_encoder::HeapType::Eq,
        wasmparser::HeapType::Struct => wasm_encoder::HeapType::Struct,
        wasmparser::HeapType::Array => wasm_encoder::HeapType::Array,
        wasmparser::HeapType::I31 => wasm_encoder::HeapType::I31,
        wasmparser::HeapType::Exn => wasm_encoder::HeapType::Exn,
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

/// Convert variant case
pub fn convert_variant_case(
    variant: wasmparser::VariantCase,
) -> (&str, Option<wasm_encoder::ComponentValType>) {
    (
        variant.name,
        match variant.ty {
            None => None,
            Some(ty) => Some(EncoderComponentValType::from(ty).ret_original()),
        },
    )
}
