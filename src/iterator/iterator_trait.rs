//! Trait that needs to be satisfied by all iterators

use crate::ir::id::GlobalID;
use crate::ir::module::module_globals::Global;
use crate::ir::types::{InstrumentationMode, Location};
use wasmparser::Operator;

#[allow(dead_code)]
/// Iterator trait that must be satisfied by all Iterators to enable code traversal.
pub trait Iterator<'a> {
    /// Reset the Iterator and all Child Iterators and SubIterators
    fn reset(&mut self);

    /// Go to the next Instruction
    fn next(&mut self) -> Option<&Operator>;

    /// Get the current location
    fn curr_loc(&self) -> Location;

    /// Get the current instruction
    fn curr_op(&self) -> Option<&Operator>;
}

/// This trait coincides with the Iterator as instrumentation occurs during Wasm visitation.
/// This trait enables code injection during traversal, thus the Iterator trait is required
/// to be implemented on the structure to inject on.
/// Instructions as defined [here].
///
/// [here]: https://webassembly.github.io/spec/core/binary/instructions.html
pub trait Instrumenter<'a>: Iterator<'a> {
    // ==== MODES ====

    /// Get the InstrumentType of the current location
    fn curr_instrument_mode(&self) -> &Option<InstrumentationMode>;

    /// Sets the type of Instrumentation Type of the current location
    fn set_instrument_mode(&mut self, ty: InstrumentationMode);

    /// Sets the type of Instrumentation Type of the specified location
    fn set_instrument_mode_at(&mut self, ty: InstrumentationMode, loc: Location);

    /// Mark the current location to InstrumentAlternate
    fn alternate(&mut self) -> &mut Self {
        self.set_instrument_mode(InstrumentationMode::Alternate);
        self
    }

    /// Mark the current location to InstrumentBefore
    fn before(&mut self) -> &mut Self {
        self.set_instrument_mode(InstrumentationMode::Before);
        self
    }

    /// Mark the current location to InstrumentAfter
    fn after(&mut self) -> &mut Self {
        self.set_instrument_mode(InstrumentationMode::After);
        self
    }

    /// Mark the current location to InstrumentSemanticAfter
    fn semantic_after(&mut self) -> &mut Self {
        self.set_instrument_mode(InstrumentationMode::SemanticAfter);
        self
    }

    // ==== INSTR INJECTION ====

    /// Splice a new instruction into a specific location
    fn add_instr_at(&mut self, loc: Location, instr: Operator<'a>);

    /// Mark the specified location to InstrumentBefore
    fn before_at(&mut self, loc: Location) -> &mut Self;

    /// Mark the specified location to InstrumentAfter
    fn after_at(&mut self, loc: Location) -> &mut Self;

    /// Mark the specified location to InstrumentAlternate
    fn alternate_at(&mut self, loc: Location) -> &mut Self;

    /// Get the instruction injected at index idx
    fn get_injected_val(&self, idx: usize) -> &Operator;

    // ==== GLOBAL INJECTION ====

    /// Adds a global to the current module and returns the Global ID
    fn add_global(&mut self, global: Global) -> GlobalID;
}
