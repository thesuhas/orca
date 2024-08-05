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
    pub fn new(params: Box<[DataType]>, results: Box<[DataType]>) -> Self {
        Self { params, results }
    }
}

#[derive(Clone, Debug)]
pub struct ModuleTypes {
    pub types: Vec<FuncType>,
}

impl ModuleTypes {
    pub fn new(types: Vec<FuncType>) -> Self {
        ModuleTypes { types }
    }

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

    pub fn len(&self) -> usize {
        self.types.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, FuncType> {
        self.types.iter()
    }

    // TODO: this is easy to be confused with get_local_func_ty
    /// Get type from index of the type section
    pub fn get(&self, index: TypeID) -> Option<&FuncType> {
        self.types.get(index as usize)
    }
}
