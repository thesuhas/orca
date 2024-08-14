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
}

impl<'a> From<wasmparser::Export<'a>> for Export {
    fn from(export: wasmparser::Export<'a>) -> Self {
        Export {
            name: export.name.to_string(),
            kind: export.kind,
            index: export.index,
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
        };
        self.exports.push(export);
    }
    /// Get export by name and return ExportID if present
    pub fn get_by_name(&self, name: String) -> Option<ExportsID> {
        for exp in self.exports.iter() {
            if exp.name == name {
                return Some(exp.index);
            }
        }
        None
    }

    pub fn get_by_id(&self, id: ExportsID) -> Option<Export> {
        for exp in self.exports.iter() {
            if exp.index == id {
                return Some(exp.clone());
            }
        }
        None
    }

    pub fn get_func_by_id(&self, id: FunctionID) -> Option<ExportsID> {
        for exp in self.exports.iter() {
            if exp.kind == ExternalKind::Func && exp.index == id {
                return Some(exp.index);
            }
        }
        None
    }

    pub fn get_func_by_name(&self, name: String) -> Option<FunctionID> {
        for exp in self.exports.iter() {
            if exp.kind == ExternalKind::Func && exp.name == name {
                return Some(exp.index);
            }
        }
        None
    }

    pub fn delete(&mut self, id: ExportsID) {
        self.exports.retain(|exp| exp.index != id)
    }
}
