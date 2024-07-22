//! Trait that needs to be satisfied by all iterators

use crate::ir::types::{DataType, InstrumentType, InstrumentationMode, Location};
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
    fn curr_instrument_type(&self) -> InstrumentType;

    /// Get the current instruction
    fn curr_op(&self) -> Option<&Operator>;

    /// Get the instruction injected at index idx
    fn get_injected_val(&self, idx: usize) -> &Operator;

    /// Sets the type of Instrumentation Type of the current location
    fn set_instrument_type(&mut self, ty: InstrumentationMode);

    /// Adds a local in the current function and returns its index
    // TODO: should we remove this since ModuleBuilder should be handling this?
    fn add_local(&mut self, val_type: DataType) -> u32;

    /// Splice a new instruction into a specific location
    fn add_instr_at(&mut self, loc: Location, instr: Operator<'a>);

    /// Mark the specified location to InstrumentBefore
    fn before_at(&mut self, loc: Location) -> &mut Self;

    /// Mark the specified location to InstrumentAfter
    fn after_at(&mut self, loc: Location) -> &mut Self;

    /// Mark the specified location to InstrumentAlternate
    fn alternate_at(&mut self, loc: Location) -> &mut Self;

    /// Sets the type of Instrumentation Type of the specified location
    fn set_instrument_type_at(&mut self, ty: InstrumentationMode, loc: Location);
}
