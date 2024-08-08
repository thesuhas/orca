//!  Intermediate representation of Module Types

use crate::ir::id::TypeID;
use crate::DataType;

/// Orca's representation of function types, shortened from [Walrus' Representation].
///
/// [Walrus' Representation]: https://docs.rs/walrus/latest/walrus/struct.Type.html
#[derive(Debug, Clone)]
pub struct FuncType {
    pub params: Box<[DataType]>,
    pub results: Box<[DataType]>,
}

impl FuncType {
    /// Create a new Function Type
    pub fn new(params: Box<[DataType]>, results: Box<[DataType]>) -> Self {
        Self { params, results }
    }
}

/// The Module Types Section
#[derive(Clone, Debug)]
pub struct ModuleTypes {
    pub types: Vec<FuncType>,
}

impl ModuleTypes {
    /// Create a new Module Types section
    pub fn new(types: Vec<FuncType>) -> Self {
        ModuleTypes { types }
    }

    /// Check if there are any types in this module
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    /// Add a new type to the module, returns the index of the new type.
    pub fn add(&mut self, param: &[DataType], ret: &[DataType]) -> TypeID {
        let index = self.types.len();
        let ty = FuncType::new(
            param.to_vec().into_boxed_slice(),
            ret.to_vec().into_boxed_slice(),
        );
        self.types.push(ty);
        index as TypeID
    }

    /// Number of types in this module
    pub fn len(&self) -> usize {
        self.types.len()
    }

    /// Create an iterable over the Type Section
    pub fn iter(&self) -> std::slice::Iter<'_, FuncType> {
        self.types.iter()
    }

    /// Get type from index of the type section
    pub fn get(&self, index: TypeID) -> Option<&FuncType> {
        self.types.get(index as usize)
    }
}
