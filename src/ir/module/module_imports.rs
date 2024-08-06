use crate::ir::id::{ExportsID, FunctionID, ImportsID};
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
}

impl<'a> From<wasmparser::Import<'a>> for Import<'a> {
    fn from(import: wasmparser::Import<'a>) -> Self {
        Import {
            module: import.module,
            name: import.name,
            ty: import.ty,
            import_name: None,
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
}

impl<'a> ModuleImports<'a> {
    pub fn new(imports: Vec<Import<'a>>) -> Self {
        ModuleImports { imports }
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

    pub fn add(&mut self, import: Import<'a>) {
        self.imports.push(import)
    }

    pub fn delete(&mut self, imports_id: ImportsID) {
        self.imports.remove(imports_id as usize);
    }

    pub fn find(&self, module: String, name: Option<String>) -> Option<ExportsID> {
        for (id, imp) in self.imports.iter().enumerate() {
            match imp.ty {
                _ => {
                    if imp.module == module.as_str() && imp.import_name == name {
                        return Some(id as ExportsID);
                    }
                }
            }
        }
        None
    }

    pub fn get_func(&self, module: String, name: Option<String>) -> Option<FunctionID> {
        for imp in self.imports.iter() {
            match imp.ty {
                TypeRef::Func(id) => {
                    if imp.module == module.as_str() && imp.import_name == name {
                        return Some(id);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn get(&self, id: ImportsID) -> &Import {
        &self.imports[id as usize]
    }
}
