//!  Intermediate representation of Module Types

use crate::ir::id::TypeID;
use crate::DataType;
use std::collections::HashMap;
use std::hash::Hash;
use wasmparser::{FieldType, PackedIndex, StorageType, UnpackedIndex};

// Orca's representation of function types, shortened from [Walrus' Representation].
//
// [Walrus' Representation]: https://docs.rs/walrus/latest/walrus/struct.Type.html

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Types {
    FuncType {
        params: Box<[DataType]>,
        results: Box<[DataType]>,
        super_type: Option<PackedIndex>,
        is_final: bool,
        shared: bool,
    },
    ArrayType {
        fields: StorageType,
        mutable: bool,
        super_type: Option<PackedIndex>,
        is_final: bool,
        shared: bool,
    },
    StructType {
        fields: Vec<StorageType>,
        mutable: Vec<bool>,
        super_type: Option<PackedIndex>,
        is_final: bool,
        shared: bool,
    },
    ContType {
        packed_index: PackedIndex,
        super_type: Option<PackedIndex>,
        is_final: bool,
        shared: bool,
    },
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

    /// Add a new function type to the module, returns the index of the new type. By default encodes the supertype as `None`, shared as `true`, and `is_final` as false for now.
    pub fn add_func_type(&mut self, param: &[DataType], ret: &[DataType]) -> TypeID {
        let index = self.types.len();
        let ty = Types::FuncType {
            params: param.to_vec().into_boxed_slice(),
            results: ret.to_vec().into_boxed_slice(),
            super_type: None,
            is_final: true,
            shared: false,
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
    ) -> TypeID {
        let index = self.types.len();
        let ty = Types::FuncType {
            params: param.to_vec().into_boxed_slice(),
            results: ret.to_vec().into_boxed_slice(),
            super_type: match super_type {
                None => None,
                Some(id) => Some(PackedIndex(*id)),
            },
            is_final,
            shared,
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
    pub fn add_array_type(&mut self, field_type: StorageType, mutable: bool) -> TypeID {
        let index = self.types.len();
        let ty = Types::ArrayType {
            fields: field_type,
            mutable,
            super_type: None,
            is_final: true,
            shared: false,
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
        field_type: StorageType,
        mutable: bool,
        super_type: Option<TypeID>,
        is_final: bool,
        shared: bool,
    ) -> TypeID {
        let index = self.types.len();
        let ty = Types::ArrayType {
            fields: field_type,
            mutable,
            super_type: match super_type {
                None => None,
                Some(id) => Some(PackedIndex(*id)),
            },
            is_final,
            shared,
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
    pub fn add_struct_type(&mut self, field_type: Vec<StorageType>, mutable: Vec<bool>) -> TypeID {
        let index = self.types.len();
        let ty = Types::StructType {
            fields: field_type,
            mutable,
            super_type: None,
            is_final: true,
            shared: false,
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
        field_type: Vec<StorageType>,
        mutable: Vec<bool>,
        super_type: Option<TypeID>,
        is_final: bool,
        shared: bool,
    ) -> TypeID {
        let index = self.types.len();
        let ty = Types::StructType {
            fields: field_type,
            mutable,
            super_type: match super_type {
                None => None,
                Some(id) => Some(PackedIndex(*id)),
            },
            is_final,
            shared,
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
