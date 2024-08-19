//! Intermediate Representation of a Function

use crate::ir::function::FunctionModifier;
use crate::ir::id::{FunctionID, ImportsID, LocalID, TypeID};
use crate::ir::types::Body;
use crate::DataType;
use std::cmp::min;

/// Represents a function. Local or Imported depends on the `FuncKind`.
#[derive(Clone, Debug)]
pub struct Function<'a> {
    pub(crate) kind: FuncKind<'a>,
    name: Option<String>,
    pub(crate) deleted: bool,
}

impl<'a> Function<'a> {
    /// Create a new function
    pub fn new(kind: FuncKind<'a>, name: Option<String>) -> Self {
        Function {
            kind,
            name,
            deleted: false,
        }
    }

    /// Get the TypeID of the function
    pub fn get_type_id(&self) -> TypeID {
        self.kind.get_type()
    }

    /// Change the kind of the Function
    pub(crate) fn set_kind(&mut self, kind: FuncKind<'a>) {
        self.kind = kind;
        // Resets deletion
        self.deleted = false;
    }

    /// Get the kind of the function
    pub fn kind(&self) -> &FuncKind<'a> {
        &self.kind
    }

    /// Check if it's a local function
    pub fn is_local(&self) -> bool {
        match &self.kind {
            FuncKind::Local(_) => true,
            _ => false,
        }
    }

    /// Check if it's an imported function
    pub fn is_import(&self) -> bool {
        match &self.kind {
            FuncKind::Import(_) => true,
            _ => false,
        }
    }

    /// Unwrap a local function. If it is an imported function, it panics.
    pub fn unwrap_local(&'a self) -> &LocalFunction<'a> {
        self.kind.unwrap_local()
    }

    pub(crate) fn delete(&mut self) {
        self.deleted = true;
    }

    /// Unwrap a local function as mutable. If it is an imported function, it panics.
    pub fn unwrap_local_mut(&mut self) -> &mut LocalFunction<'a> {
        match &mut self.kind {
            FuncKind::Local(ref mut l) => l,
            FuncKind::Import(_) => panic!("Imported Function!"),
        }
    }
}

/// Represents whether a function is a Local Function or an Imported Function
#[derive(Debug, Clone)]
pub enum FuncKind<'a> {
    Local(LocalFunction<'a>),
    Import(ImportedFunction),
}

impl<'a> FuncKind<'a> {
    /// Unwrap a local function. If it is an imported function, it panics.
    pub fn unwrap_local(&self) -> &LocalFunction<'a> {
        match &self {
            FuncKind::Local(l) => l,
            _ => panic!("Not a local function!"),
        }
    }

    /// Get the TypeID of the function
    pub fn get_type(&self) -> TypeID {
        match &self {
            FuncKind::Local(l) => l.ty_id,
            FuncKind::Import(i) => i.ty_id,
        }
    }
}

impl PartialEq for FuncKind<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FuncKind::Import(i1), FuncKind::Import(i2)) => i1.ty_id == i2.ty_id,
            (FuncKind::Local(l1), FuncKind::Local(l2)) => l1.ty_id == l2.ty_id,
            _ => false,
        }
    }
}

impl Eq for FuncKind<'_> {}

/// Intermediate Representation of a Local Function
#[derive(Clone, Debug)]
pub struct LocalFunction<'a> {
    pub ty_id: TypeID,
    pub func_id: FunctionID,
    pub body: Body<'a>,
    pub args: Vec<LocalID>,
}

impl<'a> LocalFunction<'a> {
    /// Creates a new local function
    pub fn new(
        type_id: TypeID,
        function_id: FunctionID,
        body: Body<'a>,
        args: Vec<LocalID>,
    ) -> Self {
        LocalFunction {
            ty_id: type_id,
            func_id: function_id,
            body,
            args,
        }
    }
    pub fn add_local(&mut self, ty: DataType) -> LocalID {
        add_local(
            ty,
            self.args.len(),
            &mut self.body.num_locals,
            &mut self.body.locals,
        )
    }
}

// Must split this out so that the Rust compiler knows that we're not mutating data being iterated
// over in `resolve_special_instrumentation` func.
pub(crate) fn add_local(
    ty: DataType,
    num_params: usize,
    num_locals: &mut usize,
    locals: &mut Vec<(u32, DataType)>,
) -> LocalID {
    let index = num_params + *num_locals;

    let len = locals.len();
    *num_locals += 1;
    if len > 0 {
        let last = len - 1;
        if locals[last].1 == ty {
            locals[last].0 += 1;
        } else {
            locals.push((1, ty));
        }
    } else {
        // If no locals, just append
        locals.push((1, ty));
    }

    index as LocalID
}

/// Intermediate representation of an Imported Function. The actual Import is stored in the Imports field of the module.
#[derive(Clone, Debug)]
pub struct ImportedFunction {
    pub import_id: ImportsID,            // Maps to location in a modules imports
    pub(crate) import_fn_id: FunctionID, // Maps to location in a modules imported functions
    pub ty_id: TypeID,
}

impl ImportedFunction {
    /// Create a new imported function
    pub fn new(id: ImportsID, type_id: TypeID, function_id: FunctionID) -> Self {
        ImportedFunction {
            import_id: id,
            ty_id: type_id,
            import_fn_id: function_id,
        }
    }
}

/// Intermediate representation of all the functions in a module.
#[allow(dead_code)] // may use num_import_fns in the future
#[derive(Clone, Debug)]
pub struct Functions<'a> {
    functions: Vec<Function<'a>>,
    num_import_fns: usize,
    num_local_fns: usize,
    added_local_fns: u32,
    pub(crate) deleted_local_fns: u32,
    pub(crate) first_deleted_fn: u32,
}

impl<'a> Functions<'a> {
    /// Create a new functions section
    pub fn new(functions: Vec<Function<'a>>, num_import_fns: usize, num_local_fns: usize) -> Self {
        Functions {
            functions,
            num_import_fns,
            num_local_fns,
            deleted_local_fns: 0,
            added_local_fns: 0,
            first_deleted_fn: u32::MAX,
        }
    }

    /// Get a function by its FunctionID
    pub fn get_fn_by_id(&self, function_id: FunctionID) -> Option<Function> {
        if function_id < self.functions.len() as u32 {
            return Some(self.functions[function_id as usize].clone());
        }
        None
    }

    /// Get the number of functions
    pub fn len(&self) -> usize {
        self.functions.len()
    }

    /// Add a new function
    pub fn push(&mut self, func: Function<'a>) {
        self.functions.push(func.clone());
        match func.kind {
            FuncKind::Local(_) => self.added_local_fns += 1,
            _ => {}
        }
    }

    /// Checks if there are no functions
    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }

    /// Get the type ID of a function
    pub fn get_type_id(&self, id: FunctionID) -> TypeID {
        self.functions[id as usize].get_type_id()
    }

    /// Delete a function
    pub fn delete(&mut self, id: FunctionID) {
        if id < self.functions.len() as u32 {
            self.functions[id as usize].delete();
            match self.functions[id as usize].kind {
                FuncKind::Local(_) => {
                    self.deleted_local_fns += 1;
                    self.first_deleted_fn = min(self.first_deleted_fn, id);
                }
                _ => {}
            }
        }
    }

    /// Get an iterator for the functions.
    pub fn iter(&self) -> std::slice::Iter<'_, Function<'a>> {
        self.functions.iter()
    }

    /// Get by ID
    pub fn get(&self, function_id: FunctionID) -> &Function<'a> {
        &self.functions[function_id as usize]
    }

    /// Get mutable function by ID
    pub fn get_mut(&mut self, function_id: FunctionID) -> &mut Function<'a> {
        &mut self.functions[function_id as usize]
    }

    /// Get kind of function
    pub fn get_kind(&self, function_id: FunctionID) -> &FuncKind<'a> {
        &self.functions[function_id as usize].kind
    }

    /// Check if a function is a local
    pub fn is_local(&self, function_id: FunctionID) -> bool {
        self.functions[function_id as usize].is_local()
    }

    /// Check if a function is an import
    pub fn is_import(&self, function_id: FunctionID) -> bool {
        self.functions[function_id as usize].is_import()
    }

    /// Get a function modifier from a function index
    pub fn get_fn_modifier<'b>(
        &'b mut self,
        func_id: FunctionID,
    ) -> Option<FunctionModifier<'b, 'a>> {
        // grab type and section and code section
        // let ty = self.functions.get(func_idx as usize)?;
        return match &mut self.functions.get_mut(func_id as usize)?.kind {
            FuncKind::Local(ref mut l) => Some(FunctionModifier::init(&mut l.body, &mut l.args)),
            _ => None,
        };
        // None
    }

    /// Unwrap local function. If imported, panics
    pub fn unwrap_local(&'a mut self, function_id: FunctionID) -> &mut LocalFunction<'a> {
        match &mut self.functions[function_id as usize].kind {
            FuncKind::Local(ref mut l) => l,
            FuncKind::Import(_) => panic!("Imported Function!"),
        }
    }

    /// Get local Function ID by name
    pub fn get_local_fid_by_name(&self, name: &str) -> Option<FunctionID> {
        for (idx, func) in self.functions.iter().enumerate() {
            if let FuncKind::Local(l) = &func.kind {
                match &l.body.name {
                    Some(n) => {
                        if n == name {
                            return Some(idx as FunctionID);
                        }
                    }
                    None => {}
                }
            }
        }
        None
    }

    pub(crate) fn add_import_func(
        &mut self,
        imp_id: ImportsID,
        ty_id: TypeID,
        name: Option<String>,
        imp_fn_id: u32,
    ) -> FunctionID {
        if self.num_local_fns > 0 {
            panic!("Cannot add an imported function after local functions!")
        }

        self.functions.push(Function::new(
            FuncKind::Import(ImportedFunction::new(imp_id, ty_id, imp_fn_id)),
            name,
        ));
        (self.functions.len() - 1) as FunctionID
    }

    pub(crate) fn add_local(&mut self, func_idx: FunctionID, ty: DataType) -> LocalID {
        let func_body = &mut self.functions[func_idx as usize];
        match func_body.kind {
            FuncKind::Import(_) => panic!("Imported function"),
            FuncKind::Local(_) => {}
        }

        let func = func_body.unwrap_local_mut();
        func.add_local(ty)
    }

    /// Set the name for a local function.
    pub fn set_local_fn_name(&mut self, func_idx: FunctionID, name: String) {
        match &mut self.functions[func_idx as usize].kind {
            FuncKind::Import(_) => panic!("is an imported function!"),
            FuncKind::Local(ref mut l) => l.body.name = Some(name),
        }
    }

    /// Get the name of a function
    pub fn get_name(&self, function_id: FunctionID) -> Option<String> {
        self.functions[function_id as usize].name.clone()
    }

    /// Check if it's deleted
    pub fn is_deleted(&self, function_id: FunctionID) -> bool {
        self.functions[function_id as usize].deleted
    }

    // Helper functions for rearrange
    pub(crate) fn remove(&mut self, function_id: FunctionID) -> Function<'a> {
        self.functions.remove(function_id as usize)
    }

    pub(crate) fn insert(&mut self, function_id: FunctionID, func: Function<'a>) {
        self.functions.insert(function_id as usize, func);
    }
}
