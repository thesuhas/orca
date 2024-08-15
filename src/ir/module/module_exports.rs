use crate::ir::id::{ExportsID, FunctionID};
use wasmparser::ExternalKind;

#[derive(Debug, Clone)]
pub struct Export {
    /// The name of the exported item.
    pub name: String,
    /// The kind of the export.
    pub kind: ExternalKind,
    /// The index of the exported item.
    pub index: u32,
    /// Marked for deletion
    pub(crate) deleted: bool,
}

impl<'a> From<wasmparser::Export<'a>> for Export {
    fn from(export: wasmparser::Export<'a>) -> Self {
        Export {
            name: export.name.to_string(),
            kind: export.kind,
            index: export.index,
            deleted: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModuleExports {
    exports: Vec<Export>,
}

impl ModuleExports {
    pub fn new(exports: Vec<Export>) -> Self {
        ModuleExports { exports }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Export> {
        self.exports.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.exports.is_empty()
    }

    pub fn add_export_func(&mut self, name: String, func_idx: u32) {
        let export = Export {
            name,
            kind: ExternalKind::Func,
            index: func_idx,
            deleted: false,
        };
        self.exports.push(export);
    }

    /// Get export by name and return if present
    pub fn get_by_name(&self, name: String) -> Option<Export> {
        for exp in self.exports.iter() {
            if exp.name == name {
                return Some(exp.clone());
            }
        }
        None
    }

    /// Get the export ID by name
    pub fn get_export_id_by_name(&self, name: String) -> Option<ExportsID> {
        for (idx, exp) in self.exports.iter().enumerate() {
            if exp.name == name {
                return Some(idx as ExportsID);
            }
        }
        None
    }

    /// Get the export by ID
    pub fn get_by_id(&self, id: ExportsID) -> Option<Export> {
        if id < self.exports.len() as ExportsID {
            Some(self.exports[id as usize].clone())
        } else {
            None
        }
        // for exp in self.exports.iter() {
        //     if exp.index == id {
        //         return Some(exp.clone());
        //     }
        // }
        // None
    }

    /// Get the Export ID from its function ID
    pub fn get_func_by_id(&self, id: FunctionID) -> Option<ExportsID> {
        for (export_id, exp) in self.exports.iter().enumerate() {
            match exp.kind {
                ExternalKind::Func => {
                    if exp.index == id {
                        return Some(export_id as ExportsID);
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Get the function ID of an exported function
    pub fn get_func_by_name(&self, name: String) -> Option<FunctionID> {
        for exp in self.exports.iter() {
            match exp.kind {
                ExternalKind::Func => {
                    if exp.name.to_string() == name {
                        return Some(exp.index);
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Delete an export by its exports ID
    pub fn delete(&mut self, id: ExportsID) {
        // Must just mark for deletion as or else will result in indicies getting messed up
        self.exports[id as usize].deleted = true;
    }
}
