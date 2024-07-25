//! Trait that defines general injection behaviour

use crate::ir::id::{GlobalID, LocalID};
use crate::ir::types::Global;
use crate::DataType;

pub trait ModuleBuilder {
    /// Adds a local and in the current function and returns its ID
    /// note: the implementors of this trait will keep track of the location,
    /// i.e. which function, where this local is added
    fn add_local(&mut self, ty: DataType) -> LocalID;

    /// Adds a global to the current module and returns its ID
    fn add_global(&mut self, global: Global) -> GlobalID;
}
