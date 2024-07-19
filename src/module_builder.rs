//! Trait that defines general injection behaviour

use crate::DataType;

pub trait ModuleBuilder {
    /// Adds a local and return it's index
    /// note: the implementors of this trait will keep track of the location,
    /// i.e. which function, where this local is added
    fn add_local(&mut self, ty: DataType) -> u32;
}
