//! Trait that needs to be satisfied by all iterators

use crate::ir::id::GlobalID;
use crate::ir::module::module_globals::Global;
use crate::ir::types::{InstrumentationMode, Location};
use crate::opcode::Instrumenter;
use wasmparser::Operator;

#[allow(dead_code)]
/// Iterator trait that must be satisfied by all Iterators to enable code traversal.
pub trait Iterator {
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
pub trait IteratingInstrumenter<'a>: Instrumenter<'a> + Iterator {
    // ==== MODES ====

    /// Sets the type of Instrumentation Type of the current location
    fn set_instrument_mode(&mut self, mode: InstrumentationMode);

    /// Mark the current location to InstrumentAlternate
    fn alternate(&mut self) -> &mut Self {
        self.set_instrument_mode(InstrumentationMode::Alternate);
        self
    }

    /// Insert an empty alt at the current location
    /// Effectively removes the instruction
    fn empty_alternate(&mut self) -> &mut Self {
        self.empty_alternate_at(self.curr_loc());
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

    /// Mark the current location to InstrumentBlockEntry
    fn block_entry(&mut self) -> &mut Self {
        self.set_instrument_mode(InstrumentationMode::BlockEntry);
        self
    }

    /// Mark the current location to InstrumentBlockExit
    fn block_exit(&mut self) -> &mut Self {
        self.set_instrument_mode(InstrumentationMode::BlockExit);
        self
    }

    /// Mark the current location to InstrumentBlockAlt
    fn block_alt(&mut self) -> &mut Self {
        self.set_instrument_mode(InstrumentationMode::BlockAlt);
        self
    }

    /// Insert an empty alt block at the current location
    /// Effectively removes the block
    fn empty_block_alt(&mut self) -> &mut Self {
        self.empty_block_alt_at(self.curr_loc());
        self
    }

    // ==== VAR INJECTION ====

    /// Adds a global to the current module and returns the Global ID
    fn add_global(&mut self, global: Global) -> GlobalID;
}
