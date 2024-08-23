//! The Intermediate Representation for components and modules.

pub mod component;
pub mod function;
mod helpers;
pub mod id;
#[cfg(test)]
pub mod instr_tests;
pub mod module;
pub mod section;
pub mod types;
pub(crate) mod wrappers;
mod dwarf;
