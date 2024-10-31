//!  Intermediate representation of Module Types

use crate::ir::id::TypeID;
use crate::DataType;
use std::collections::HashMap;
use std::hash::Hash;
use wasmparser::FieldType;

// Orca's representation of function types, shortened from [Walrus' Representation].
//
// [Walrus' Representation]: https://docs.rs/walrus/latest/walrus/struct.Type.html

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Types {
    FuncType {
        params: Box<[DataType]>,
        results: Box<[DataType]>,
    },
    ArrayType {
        fields: FieldType,
        mutable: bool,
    },
    StructType {
        fields: Vec<FieldType>,
        mutable: bool,
    },
}

impl Types {
    pub fn params(&self) -> Vec<DataType> {
        match &self {
            Types::FuncType { params, .. } => params.to_vec(),
            _ => panic!("Not a function!"),
        }
    }

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
}

impl ModuleTypes {
    /// Create a new Module Types section
    pub fn new(types: Vec<Types>) -> Self {
        let mut types_map = HashMap::default();
        for (id, ty) in types.iter().enumerate() {
            types_map.insert(ty.clone(), TypeID(id as u32));
        }
        ModuleTypes { types, types_map }
    }

    /// Check if there are any types in this module
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    /// Add a new type to the module, returns the index of the new type.
    pub fn add(&mut self, param: &[DataType], ret: &[DataType]) -> TypeID {
        let index = self.types.len();
        let ty = Types::FuncType {
            params: param.to_vec().into_boxed_slice(),
            results: ret.to_vec().into_boxed_slice(),
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
