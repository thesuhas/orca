//!  Intermediate representation of Module Types

use crate::ir::id::TypeID;
use crate::ir::types::{InjectTag, Tag, TagUtils};
use crate::DataType;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use wasmparser::{PackedIndex, UnpackedIndex};

/// Orca's representation of types, initally shortened from [Walrus' Representation] but now extended to support WASM GC.
///
/// [Walrus' Representation]: https://docs.rs/walrus/latest/walrus/struct.Type.html
#[derive(Clone, Debug, Eq)]
pub enum Types {
    FuncType {
        params: Box<[DataType]>,
        results: Box<[DataType]>,
        super_type: Option<PackedIndex>,
        is_final: bool,
        shared: bool,
        tag: InjectTag,
    },
    ArrayType {
        fields: DataType,
        mutable: bool,
        super_type: Option<PackedIndex>,
        is_final: bool,
        shared: bool,
        tag: InjectTag,
    },
    StructType {
        fields: Vec<DataType>,
        mutable: Vec<bool>,
        super_type: Option<PackedIndex>,
        is_final: bool,
        shared: bool,
        tag: InjectTag,
    },
    ContType {
        packed_index: PackedIndex,
        super_type: Option<PackedIndex>,
        is_final: bool,
        shared: bool,
        tag: InjectTag,
    },
}
impl Hash for Types {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // custom implementation to skip hashing the tag!
        state.write_u8(self.hash_id());
        match self {
            Types::FuncType { params, results, super_type, is_final, shared, .. } => {
                params.hash(state);
                results.hash(state);
                super_type.hash(state);
                is_final.hash(state);
                shared.hash(state);
            }
            Types::ArrayType { fields, mutable, super_type, is_final, shared, .. } => {
                fields.hash(state);
                mutable.hash(state);
                super_type.hash(state);
                is_final.hash(state);
                shared.hash(state);
            }
            Types::StructType { fields, mutable, super_type, is_final, shared, .. } => {
                fields.hash(state);
                mutable.hash(state);
                super_type.hash(state);
                is_final.hash(state);
                shared.hash(state);
            }
            Types::ContType { packed_index, super_type, is_final, shared, .. } => {
                packed_index.hash(state);
                super_type.hash(state);
                is_final.hash(state);
                shared.hash(state);
            }
        }
    }
}
impl PartialEq for Types {
    fn eq(&self, other: &Self) -> bool {
        // custom implementation to skip checking the tag!
        match (self, other) {
            (Self::FuncType {params, results, super_type, is_final, shared, ..}, Self::FuncType {params: params1, results: results1, super_type: super_type1, is_final: is_final1, shared: shared1, ..}) => {
                params.eq(params1) && results.eq(results1) && super_type.eq(super_type1) && *is_final == *is_final1 && *shared == *shared1
            }
            (Self::ArrayType {fields, mutable, super_type, is_final, shared, ..}, Self::ArrayType {fields: fields1, mutable: mutable1, super_type: super_type1, is_final: is_final1, shared: shared1, ..}) => {
                fields.eq(fields1) && *mutable == *mutable1 && super_type.eq(super_type1) && *is_final == *is_final1 && *shared == *shared1
            }
            (Self::StructType {fields, mutable, super_type, is_final, shared, ..}, Self::StructType {fields: fields1, mutable: mutable1, super_type: super_type1, is_final: is_final1, shared: shared1, ..}) => {
                fields.eq(fields1) && *mutable == *mutable1 && super_type.eq(super_type1) && *is_final == *is_final1 && *shared == *shared1
            }
            (Self::ContType {packed_index, super_type, is_final, shared, ..}, Self::ContType {packed_index: packed_index1, super_type: super_type1, is_final: is_final1, shared: shared1, ..}) => {
                packed_index.eq(packed_index1) && super_type.eq(super_type1) && *is_final == *is_final1 && *shared == *shared1
            }
            (_, _) => false
        }
    }
}
impl TagUtils for Types {
    fn get_or_create_tag(&mut self) -> &mut Tag {
        match self {
            Types::FuncType { tag, .. }
            | Types::ArrayType { tag, .. }
            | Types::StructType { tag, .. }
            | Types::ContType { tag, .. } => tag.get_or_insert_default(),
        }
    }

    fn get_tag(&self) -> &Option<Tag> {
        match self {
            Types::FuncType { tag, .. }
            | Types::ArrayType { tag, .. }
            | Types::StructType { tag, .. }
            | Types::ContType { tag, .. } => tag,
        }
    }
}
impl Types {
    /// Return the params of a Function Type
    pub fn params(&self) -> Vec<DataType> {
        match &self {
            Types::FuncType { params, .. } => params.to_vec(),
            _ => panic!("Not a function!"),
        }
    }

    /// Return the params of a Function Type
    pub fn results(&self) -> Vec<DataType> {
        match &self {
            Types::FuncType { results, .. } => results.to_vec(),
            _ => panic!("Not a function!"),
        }
    }

    fn hash_id(&self) -> u8 {
        match self {
            Types::FuncType { .. } => 0,
            Types::ArrayType { .. } => 1,
            Types::StructType { .. } => 2,
            Types::ContType { .. } => 3
        }
    }
}

/// The Module Types Section
#[derive(Clone, Debug, Default)]
pub struct ModuleTypes {
    pub types: Vec<Types>,
    /// This enables us to quickly do a lookup to determine if a type has already been added
    pub types_map: HashMap<Types, TypeID>,
    // Mapping between recursive group and TypeID
    pub(crate) recgroup_map: HashMap<u32, u32>,
}

impl ModuleTypes {
    /// Create a new Module Types section
    pub fn new(types: Vec<Types>, recgroup_map: HashMap<u32, u32>) -> Self {
        let mut types_map = HashMap::default();
        for (id, ty) in types.iter().enumerate() {
            types_map.insert(ty.clone(), TypeID(id as u32));
        }
        ModuleTypes {
            types,
            types_map,
            recgroup_map,
        }
    }

    /// Check if there are any types in this module
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    /// Add a new function type to the module, returns the index of the new type. By default, encodes the supertype as `None`, shared as `true`, and `is_final` as false for now.
    pub fn add_func_type(
        &mut self,
        param: &[DataType],
        ret: &[DataType],
        tag: InjectTag,
    ) -> TypeID {
        let index = self.types.len();
        let ty = Types::FuncType {
            params: param.to_vec().into_boxed_slice(),
            results: ret.to_vec().into_boxed_slice(),
            super_type: None,
            is_final: true,
            shared: false,
            tag,
        };

        if !self.types_map.contains_key(&ty) {
            self.types.push(ty.clone());
        }
        *self
            .types_map
            .entry(ty.clone())
            .or_insert(TypeID(index as u32))
    }

    /// Add a new function type to the module with all parameters.
    pub fn add_func_type_with_params(
        &mut self,
        param: &[DataType],
        ret: &[DataType],
        super_type: Option<TypeID>,
        is_final: bool,
        shared: bool,
        tag: InjectTag,
    ) -> TypeID {
        let index = self.types.len();
        let ty = Types::FuncType {
            params: param.to_vec().into_boxed_slice(),
            results: ret.to_vec().into_boxed_slice(),
            super_type: match super_type {
                None => None,
                Some(id) => PackedIndex::from_module_index(*id),
            },
            is_final,
            shared,
            tag,
        };

        if !self.types_map.contains_key(&ty) {
            self.types.push(ty.clone());
        }
        *self
            .types_map
            .entry(ty.clone())
            .or_insert(TypeID(index as u32))
    }

    /// Add a new array type to the module. Assumes no `super_type` and `is_final` is `true`
    pub fn add_array_type(
        &mut self,
        field_type: DataType,
        mutable: bool,
        tag: InjectTag,
    ) -> TypeID {
        let index = self.types.len();
        let ty = Types::ArrayType {
            fields: field_type,
            mutable,
            super_type: None,
            is_final: true,
            shared: false,
            tag,
        };

        if !self.types_map.contains_key(&ty) {
            self.types.push(ty.clone());
        }

        *self
            .types_map
            .entry(ty.clone())
            .or_insert(TypeID(index as u32))
    }

    /// Add a new array type with all parameters.
    pub fn add_array_type_with_params(
        &mut self,
        field_type: DataType,
        mutable: bool,
        super_type: Option<TypeID>,
        is_final: bool,
        shared: bool,
        tag: InjectTag,
    ) -> TypeID {
        let index = self.types.len();
        let ty = Types::ArrayType {
            fields: field_type,
            mutable,
            super_type: match super_type {
                None => None,
                Some(id) => PackedIndex::from_module_index(*id),
            },
            is_final,
            shared,
            tag,
        };

        if !self.types_map.contains_key(&ty) {
            self.types.push(ty.clone());
        }

        *self
            .types_map
            .entry(ty.clone())
            .or_insert(TypeID(index as u32))
    }

    /// Add a new struct type to the module. Assumes no `super_type` and `is_final` is `true`
    pub fn add_struct_type(
        &mut self,
        field_type: Vec<DataType>,
        mutable: Vec<bool>,
        tag: InjectTag,
    ) -> TypeID {
        let index = self.types.len();
        let ty = Types::StructType {
            fields: field_type,
            mutable,
            super_type: None,
            is_final: true,
            shared: false,
            tag,
        };

        if !self.types_map.contains_key(&ty) {
            self.types.push(ty.clone());
        }

        *self
            .types_map
            .entry(ty.clone())
            .or_insert(TypeID(index as u32))
    }

    /// Add a new array type with all parameters.
    pub fn add_struct_type_with_params(
        &mut self,
        field_type: Vec<DataType>,
        mutable: Vec<bool>,
        super_type: Option<TypeID>,
        is_final: bool,
        shared: bool,
        tag: InjectTag,
    ) -> TypeID {
        let index = self.types.len();
        let ty = Types::StructType {
            fields: field_type,
            mutable,
            super_type: match super_type {
                None => None,
                Some(id) => PackedIndex::from_module_index(*id),
            },
            is_final,
            shared,
            tag,
        };

        if !self.types_map.contains_key(&ty) {
            self.types.push(ty.clone());
        }

        *self
            .types_map
            .entry(ty.clone())
            .or_insert(TypeID(index as u32))
    }

    /// Number of types in this module
    pub fn len(&self) -> usize {
        self.types.len()
    }

    /// Create an iterable over the Type Section
    pub fn iter(&self) -> std::slice::Iter<'_, Types> {
        self.types.iter()
    }

    /// Get type from index of the type section
    pub fn get(&self, index: TypeID) -> Option<&Types> {
        self.types.get(*index as usize)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum HeapType {
    Abstract { shared: bool, ty: AbstractHeapType },
    // TODO: See to replace UnpackedIndex with `orca` specific implementation
    Concrete(UnpackedIndex),
}

impl From<wasmparser::HeapType> for HeapType {
    fn from(value: wasmparser::HeapType) -> Self {
        match value {
            wasmparser::HeapType::Abstract { shared, ty } => HeapType::Abstract {
                shared,
                ty: AbstractHeapType::from(ty),
            },
            wasmparser::HeapType::Concrete(idx) => HeapType::Concrete(idx),
        }
    }
}

impl From<HeapType> for wasmparser::HeapType {
    fn from(value: HeapType) -> Self {
        match value {
            HeapType::Abstract { shared, ty } => wasmparser::HeapType::Abstract {
                shared,
                ty: wasmparser::AbstractHeapType::from(ty),
            },
            HeapType::Concrete(idx) => wasmparser::HeapType::Concrete(idx),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AbstractHeapType {
    Func,
    Extern,
    Any,
    None,
    NoExtern,
    NoFunc,
    Eq,
    Struct,
    Array,
    I31,
    Exn,
    NoExn,
    Cont,
    NoCont,
}

impl From<wasmparser::AbstractHeapType> for AbstractHeapType {
    fn from(value: wasmparser::AbstractHeapType) -> Self {
        match value {
            wasmparser::AbstractHeapType::Func => AbstractHeapType::Func,
            wasmparser::AbstractHeapType::Extern => AbstractHeapType::Extern,
            wasmparser::AbstractHeapType::Any => AbstractHeapType::Any,
            wasmparser::AbstractHeapType::None => AbstractHeapType::None,
            wasmparser::AbstractHeapType::NoExtern => AbstractHeapType::NoExtern,
            wasmparser::AbstractHeapType::NoFunc => AbstractHeapType::NoFunc,
            wasmparser::AbstractHeapType::Eq => AbstractHeapType::Eq,
            wasmparser::AbstractHeapType::Struct => AbstractHeapType::Struct,
            wasmparser::AbstractHeapType::Array => AbstractHeapType::Array,
            wasmparser::AbstractHeapType::I31 => AbstractHeapType::I31,
            wasmparser::AbstractHeapType::Exn => AbstractHeapType::Exn,
            wasmparser::AbstractHeapType::NoExn => AbstractHeapType::NoExn,
            wasmparser::AbstractHeapType::Cont => AbstractHeapType::Cont,
            wasmparser::AbstractHeapType::NoCont => AbstractHeapType::NoCont,
        }
    }
}

impl From<AbstractHeapType> for wasmparser::AbstractHeapType {
    fn from(value: AbstractHeapType) -> Self {
        match value {
            AbstractHeapType::Func => wasmparser::AbstractHeapType::Func,
            AbstractHeapType::Extern => wasmparser::AbstractHeapType::Extern,
            AbstractHeapType::Any => wasmparser::AbstractHeapType::Any,
            AbstractHeapType::None => wasmparser::AbstractHeapType::None,
            AbstractHeapType::NoExtern => wasmparser::AbstractHeapType::NoExtern,
            AbstractHeapType::NoFunc => wasmparser::AbstractHeapType::NoFunc,
            AbstractHeapType::Eq => wasmparser::AbstractHeapType::Eq,
            AbstractHeapType::Struct => wasmparser::AbstractHeapType::Struct,
            AbstractHeapType::Array => wasmparser::AbstractHeapType::Array,
            AbstractHeapType::I31 => wasmparser::AbstractHeapType::I31,
            AbstractHeapType::Exn => wasmparser::AbstractHeapType::Exn,
            AbstractHeapType::NoExn => wasmparser::AbstractHeapType::NoExn,
            AbstractHeapType::Cont => wasmparser::AbstractHeapType::Cont,
            AbstractHeapType::NoCont => wasmparser::AbstractHeapType::NoCont,
        }
    }
}
