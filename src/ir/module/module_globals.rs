//! Intermediate representation of the globals.

use crate::error::Error;
use crate::ir::id::{GlobalID, ImportsID};
use crate::ir::module::module_imports::ModuleImports;
use crate::ir::module::{GetID, Iter, LocalOrImport, ReIndexable};
use crate::InitExpr;
use std::vec::IntoIter;
use wasmparser::{GlobalType, TypeRef};

type Result<T> = std::result::Result<T, Error>;

/// Represents whether a Global is Local or Imported
#[derive(Clone, Debug)]
pub enum GlobalKind {
    Local(LocalGlobal),
    Import(ImportedGlobal),
}

/// Represents a global that is locally defined in the module.
#[derive(Clone, Debug)]
pub struct LocalGlobal {
    pub global_id: GlobalID,
    pub ty: GlobalType,
    pub init_expr: InitExpr,
}

/// Represents a global that is imported into the module.
#[derive(Clone, Debug)]
pub struct ImportedGlobal {
    pub import_id: ImportsID, // Maps to location in a modules imports
    pub(crate) import_global_id: GlobalID, // Maps to location in a modules imported globals
    pub ty: GlobalType,
}

impl ImportedGlobal {
    pub(crate) fn new(import_id: ImportsID, import_global_id: GlobalID, ty: GlobalType) -> Self {
        Self {
            import_id,
            import_global_id,
            ty,
        }
    }
}

/// Globals in a wasm module.
#[derive(Debug, Clone)]
pub struct Global {
    /// The kind of global (imported or locally-defined).
    pub(crate) kind: GlobalKind,
    /// Whether this global was deleted.
    pub(crate) deleted: bool,
}

impl GetID for Global {
    /// Get the ID of the global
    fn get_id(&self) -> u32 {
        match &self.kind {
            GlobalKind::Local(LocalGlobal { global_id, .. })
            | GlobalKind::Import(ImportedGlobal {
                import_global_id: global_id,
                ..
            }) => **global_id,
        }
    }
}

impl LocalOrImport for Global {
    /// Returns whether this global is locally defined (not imported).
    fn is_local(&self) -> bool {
        matches!(&self.kind, GlobalKind::Local(_))
    }

    /// Returns whether this global is imported.
    fn is_import(&self) -> bool {
        matches!(&self.kind, GlobalKind::Import(_))
    }

    /// Check if this global has been deleted
    fn is_deleted(&self) -> bool {
        self.deleted
    }
}

impl Global {
    pub fn new(kind: GlobalKind) -> Self {
        Self {
            kind,
            deleted: false,
        }
    }

    /// Convert from wasmparser Global representation to Orca's representation.
    /// Assumes this is a locally-defined global (not imported).
    pub(crate) fn from_wasmparser(global: wasmparser::Global) -> Result<Global> {
        let ty = global.ty;
        let init_expr = InitExpr::eval(&global.init_expr);
        Ok(Global {
            kind: GlobalKind::Local(LocalGlobal {
                global_id: GlobalID(0),
                ty,
                init_expr,
            }),
            deleted: false,
        })
    }

    pub(crate) fn set_id(&mut self, id: GlobalID) {
        match &mut self.kind {
            GlobalKind::Local(LocalGlobal { global_id, .. })
            | GlobalKind::Import(ImportedGlobal {
                import_global_id: global_id,
                ..
            }) => {
                *global_id = id;
            }
        }
    }

    fn delete(&mut self) {
        self.deleted = true;
    }
}

/// The globals section of a module
#[derive(Clone, Debug, Default)]
pub struct ModuleGlobals {
    globals: Vec<Global>,
}

impl Iter<Global> for ModuleGlobals {
    /// Get an iterator for the functions.
    fn iter(&self) -> std::slice::Iter<'_, Global> {
        self.globals.iter()
    }

    fn get_into_iter(&self) -> IntoIter<Global> {
        self.globals.clone().into_iter()
    }
}

impl ReIndexable<Global> for ModuleGlobals {
    /// Get the number of functions
    fn len(&self) -> usize {
        self.globals.len()
    }
    fn remove(&mut self, global_id: u32) -> Global {
        self.globals.remove(global_id as usize)
    }

    fn insert(&mut self, global_id: u32, global: Global) {
        self.globals.insert(global_id as usize, global);
    }
    /// Add a new function
    fn push(&mut self, global: Global) {
        self.globals.push(global);
    }
}

impl ModuleGlobals {
    /// Create a new globals section
    pub fn new(imports: &ModuleImports, local_globals: Vec<Global>) -> Self {
        let mut result = ModuleGlobals::default();

        // Add the imported globals
        let mut curr_global_id: u32 = 0;
        for (id, import) in imports.iter().enumerate() {
            if let TypeRef::Global(ty) = import.ty {
                curr_global_id += 1;
                // This is an imported global
                result.add(Global {
                    kind: GlobalKind::Import(ImportedGlobal {
                        import_id: ImportsID(id as u32),
                        import_global_id: GlobalID(curr_global_id),
                        ty,
                    }),
                    deleted: false,
                });
            };
        }

        // Add the locally defined globals and fix the IDs
        for global in local_globals.iter() {
            // fix the ID
            let mut owned = global.to_owned();
            owned.set_id(GlobalID(curr_global_id));
            curr_global_id += 1;

            result.add(owned);
        }
        result
    }

    /// Get kind of global
    pub fn get_kind(&self, global_id: GlobalID) -> &GlobalKind {
        &self.globals[*global_id as usize].kind
    }

    /// Create an iterable over the global section
    pub fn iter(&self) -> std::slice::Iter<'_, Global> {
        self.globals.iter()
    }

    /// Get the number of globals
    pub fn len(&self) -> usize {
        self.globals.len()
    }

    /// Check if there are any globals
    pub fn is_empty(&self) -> bool {
        self.globals.is_empty()
    }

    /// Remove the last global from the list. Can only remove the final Global due to indexing
    pub(crate) fn delete(&mut self, id: GlobalID) {
        if *id < self.globals.len() as u32 {
            self.globals[*id as usize].delete();
        }
    }

    /// Add a new Global to the module. Returns the index of the new Global.
    pub(crate) fn add(&mut self, mut global: Global) -> GlobalID {
        let id = GlobalID(self.globals.len() as u32);
        global.set_id(id);
        self.globals.push(global);
        id
    }
}
