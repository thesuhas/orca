//! Iterators to traverse either a Component or a Module. Supports injection of Instructions at specific locations.

/// Iterator to traverse a Component
pub mod component_iterator;
/// Trait that needs to be satisfied by all iterators
pub mod iterator_trait;
/// Iterator to traverse a Module
pub mod module_iterator;
