//! Intermediate Representation of a Module's Imports

use crate::ir::id::{FunctionID, ImportsID};
use wasmparser::TypeRef;

// TODO: Need to handle the relationship between Functions and Imports
/// Represents an import in a WebAssembly module.
#[derive(Debug, Clone)]
pub struct Import<'a> {
    /// The module being imported from.
    pub module: &'a str,
    /// The name of the imported item.
    pub name: &'a str,
    /// The type of the imported item.
    pub ty: TypeRef,
    /// The name (in the custom section) of the imported item.
    pub custom_name: Option<String>,
    pub(crate) deleted: bool,
}

impl<'a> From<wasmparser::Import<'a>> for Import<'a> {
    fn from(import: wasmparser::Import<'a>) -> Self {
        Import {
            module: import.module,
            name: import.name,
            ty: import.ty,
            custom_name: None,
            deleted: false,
        }
    }
}

impl Import<'_> {
    // TODO: Add documentation here
    pub fn is_function(&self) -> bool {
        matches!(self.ty, TypeRef::Func(_))
    }
}

#[derive(Clone, Debug)]
/// Represents the Imports Section of a WASM Module
pub struct ModuleImports<'a> {
    imports: Vec<Import<'a>>,

    pub(crate) num_funcs: u32,
    // Variables representing functions added/deleted
    // pub(crate) deleted_imports: u32,
    // pub(crate) added_imports: u32,
    // pub(crate) first_deleted_import: u32,
}

impl<'a> ModuleImports<'a> {
    /// Creates a new `ModuleImports` struct given a Vec of Imports
    pub fn new(imports: Vec<Import<'a>>) -> Self {
        ModuleImports {
            imports,
            num_funcs: 0,
            // deleted_imports: 0,
            // added_imports: 0,
            // first_deleted_import: u32::MAX,
        }
    }

    /// Checks if there are no Imports
    pub fn is_empty(&self) -> bool {
        self.imports.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Import<'a>> {
        self.imports.iter()
    }

    /// Set the name of a given import
    pub fn set_name(&mut self, name: String, imports_id: ImportsID) {
        self.imports[imports_id as usize].custom_name = Some(name)
    }

    /// Returns the number of imports
    pub fn len(&self) -> usize {
        self.imports.len()
    }

    pub(crate) fn add<'b>(&'b mut self, import: Import<'a>) -> ImportsID {
        self.imports.push(import);
        (self.imports.len() - 1) as ImportsID
    }

    pub(crate) fn add_func<'b>(&'b mut self, import: Import<'a>) -> ImportsID {
        self.num_funcs += 1;
        self.add(import)
    }

    pub(crate) fn delete(&mut self, imports_id: ImportsID) {
        self.imports[imports_id as usize].deleted = true;
    }

    /// Find an import by the `module` and `name` and return its `ImportsID` if found
    pub fn find(&self, module: String, name: String) -> Option<ImportsID> {
        for (id, imp) in self.imports.iter().enumerate() {
            if imp.module == module.as_str() && imp.name == name.as_str() {
                return Some(id as ImportsID);
            }
        }
        None
    }

    /// Get the function ID of an Imported Function
    pub fn get_func(&self, module: String, name: String) -> Option<FunctionID> {
        for (idx, imp) in self.imports.iter().enumerate() {
            if let TypeRef::Func(_) = imp.ty {
                if imp.module == module.as_str() && *imp.name == name {
                    return Some(idx as FunctionID);
                }
            }
        }
        None
    }

    /// Get an Import by its `ImportsID`
    pub fn get(&self, id: ImportsID) -> &Import {
        &self.imports[id as usize]
    }

    /// Get the name of an Import
    pub fn get_import_name(&self, imports_id: ImportsID) -> &Option<String> {
        &self.imports[imports_id as usize].custom_name
    }
}
