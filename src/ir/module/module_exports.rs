use crate::ir::id::{ExportsID, FunctionID};
use wasmparser::{Export, ExternalKind};

#[derive(Clone, Debug)]
pub struct ModuleExports<'a> {
    exports: Vec<Export<'a>>,
}

impl<'a> ModuleExports<'a> {
    pub fn new(exports: Vec<Export<'a>>) -> Self {
        ModuleExports { exports }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Export<'a>> {
        self.exports.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.exports.is_empty()
    }

    pub fn add_export_func(&mut self, name: &'a str, func_idx: u32) {
        let export = Export {
            name,
            kind: wasmparser::ExternalKind::Func,
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

    pub fn get_func_by_id(&self, id: FunctionID) -> Option<Export> {
        for exp in self.exports.iter() {
            match exp.kind {
                ExternalKind::Func => {
                    if exp.index == id {
                        return Some(exp.clone());
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn get_func_by_name(&self, name: String) -> Option<Export> {
        for exp in self.exports.iter() {
            match exp.kind {
                ExternalKind::Func => {
                    if exp.name.to_string() == name {
                        return Some(exp.clone());
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn delete(&mut self, id: ExportsID) {
        self.exports.retain(|exp| exp.index != id)
    }
}
