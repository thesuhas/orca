use crate::error::Error;
use crate::ir::id::GlobalID;
use crate::InitExpr;
use wasmparser::GlobalType;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
/// Globals in a wasm module.
pub struct Global {
    pub ty: GlobalType,
    pub init_expr: InitExpr,
}

impl Global {
    pub(crate) fn from_wasmparser(global: wasmparser::Global) -> Result<Global> {
        let ty = global.ty;
        let init_expr = InitExpr::eval(&global.init_expr);
        Ok(Global { ty, init_expr })
    }
}

#[derive(Clone, Debug)]
pub struct ModuleGlobals {
    globals: Vec<Global>,
}

impl ModuleGlobals {
    pub fn new(globals: Vec<Global>) -> Self {
        ModuleGlobals { globals }
    }

    pub fn is_empty(&self) -> bool {
        self.globals.is_empty()
    }

    /// Remove the last global from the list. Can only remove the final Global due to indexing
    pub fn remove(&mut self) {
        if !self.globals.is_empty() {
            self.globals.pop();
        }
    }

    /// Add a new Global to the module. Returns the index of the new Global.
    pub fn add(&mut self, global: Global) -> GlobalID {
        let index = self.globals.len();
        self.globals.push(global);
        index as GlobalID
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Global> {
        self.globals.iter()
    }
}
