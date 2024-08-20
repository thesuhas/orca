use crate::ir::id::{FunctionID, ImportsID};
use std::cmp::min;
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
    pub ty: wasmparser::TypeRef,
    /// The name (in the custom section) of the imported item.
    pub import_name: Option<String>,
    pub(crate) deleted: bool,
}

impl<'a> From<wasmparser::Import<'a>> for Import<'a> {
    fn from(import: wasmparser::Import<'a>) -> Self {
        Import {
            module: import.module,
            name: import.name,
            ty: import.ty,
            import_name: None,
            deleted: false,
        }
    }
}

impl Import<'_> {
    // TODO: Add documentation here
    pub fn is_function(&self) -> bool {
        matches!(self.ty, wasmparser::TypeRef::Func(_))
    }
}

#[derive(Clone, Debug)]
pub struct ModuleImports<'a> {
    imports: Vec<Import<'a>>,

    pub(crate) num_funcs: u32,

    // Variables representing functions added/deleted
    pub(crate) deleted_imports: u32,
    pub(crate) added_imports: u32,
    pub(crate) first_deleted_import: u32,
}

impl<'a> ModuleImports<'a> {
    pub fn new(imports: Vec<Import<'a>>) -> Self {
        ModuleImports {
            imports,
            num_funcs: 0,
            deleted_imports: 0,
            added_imports: 0,
            first_deleted_import: u32::MAX,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.imports.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Import<'a>> {
        self.imports.iter()
    }

    pub fn set_name(&mut self, name: String, imports_id: ImportsID) {
        self.imports[imports_id as usize].import_name = Some(name)
    }

    pub fn len(&self) -> usize {
        self.imports.len()
    }

    pub(crate) fn add(&mut self, import: Import<'a>) {
        self.imports.push(import);
        self.added_imports += 1;
    }

    pub(crate) fn add_func(&mut self, import: Import<'a>) {
        self.add(import);
        self.num_funcs += 1;
    }

    pub(crate) fn delete(&mut self, imports_id: ImportsID) {
        self.imports[imports_id as usize].deleted = true;
        self.deleted_imports += 1;
        self.first_deleted_import = min(self.first_deleted_import, imports_id);
    }

    pub fn find(&self, module: String, name: String) -> Option<ImportsID> {
        for (id, imp) in self.imports.iter().enumerate() {
            if imp.module == module.as_str() && imp.name == name.as_str() {
                return Some(id as ImportsID);
            }
        }
        None
    }

    pub fn get_func(&self, module: String, name: Option<String>) -> Option<FunctionID> {
        for imp in self.imports.iter() {
            if let TypeRef::Func(id) = imp.ty {
                if imp.module == module.as_str() && imp.import_name == name {
                    return Some(id);
                }
            }
        }
        None
    }

    pub fn get(&self, id: ImportsID) -> &Import {
        &self.imports[id as usize]
    }

    pub fn get_import_name(&self, imports_id: ImportsID) -> &Option<String> {
        &self.imports[imports_id as usize].import_name
    }
}
