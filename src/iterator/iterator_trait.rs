//! Trait that needs to be satisfied by all iterators

use crate::ir::types::{InstrumentType, Location};
use wasmparser::Operator;

#[allow(dead_code)]
/// Iterator trait that must be satisfied by all Iterators. Enables code injection and traversal.
/// Instructions as defined [here].
///
/// [here]: https://webassembly.github.io/spec/core/binary/instructions.html
pub trait Iterator<'a> {
    /// Mark the current location to InstrumentBefore
    fn before(&mut self) -> &mut Self;

    /// Mark the current location to InstrumentAfter
    fn after(&mut self) -> &mut Self;

    /// Mark the current location to InstrumentAlternate
    fn alternate(&mut self) -> &mut Self;

    /// Reset the Iterator and all Child Iterators and SubIterators
    fn reset(&mut self);

    /// Go to the next Instruction
    fn next(&mut self) -> Option<&Operator>;

    /// Get the current location
    fn curr_loc(&self) -> Location;

    /// Get the InstrumentType of the current location
    fn curr_instrument_type(&self) -> &InstrumentType;

    /// Get the current instruction
    fn curr_op(&self) -> Option<&Operator>;

    /// Get the instruction injected at index idx
    fn get_injected_val(&self, idx: usize) -> &Operator;
}
