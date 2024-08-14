//! Trait that needs to be satisfied by all iterators

use crate::ir::id::GlobalID;
use crate::ir::module::module_functions::LocalFunction;
use crate::ir::module::module_globals::Global;
use crate::ir::types::{InstrumentType, InstrumentationMode, Location, Instrument};
use wasmparser::Operator;

pub trait Instrumenter<'a> {
    // ==== MODES ====

    /// Get the InstrumentType of the current location
    fn curr_instrument_type(&self) -> InstrumentType;

    /// Sets the type of Instrumentation Type of the current location
    fn set_instrument_type(&mut self, ty: InstrumentationMode);

    /// Sets the type of Instrumentation Type of the specified location
    fn set_instrument_type_at(&mut self, ty: InstrumentationMode, loc: Location);

    /// Mark the current location to InstrumentBefore
    fn before(&mut self) -> &mut Self {
        self.set_instrument_type(InstrumentationMode::Before);
        self
    }

    /// Mark the current location to InstrumentAfter
    fn after(&mut self) -> &mut Self {
        self.set_instrument_type(InstrumentationMode::After);
        self
    }

    /// Mark the current location to InstrumentAlternate
    fn alternate(&mut self) -> &mut Self {
        self.set_instrument_type(InstrumentationMode::Alternate);
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

// Helper functions to reduce duplicate code, cannot be member of trait due to E0599
pub(crate) fn set_instrument_type_for_local_func_at(mode: InstrumentationMode, func: &mut LocalFunction, instr_idx: usize) {
    if func.body.instructions[instr_idx].1.get_curr()
        == InstrumentType::NotInstrumented
    {
        func.body.instructions[instr_idx].1 = Instrument::Instrumented {
            before: vec![],
            after: vec![],
            alternate: vec![],
            current: mode,
        }
    } else {
        func.body.instructions[instr_idx].1.set_curr(mode);
    }
}

#[allow(dead_code)]
/// Iterator trait that must be satisfied by all Iterators. Enables code injection and traversal.
/// Instructions as defined [here].
///
/// [here]: https://webassembly.github.io/spec/core/binary/instructions.html
pub trait Iterator<'a>: Instrumenter<'a> {
    /// Reset the Iterator and all Child Iterators and SubIterators
    fn reset(&mut self);

    /// Go to the next Instruction
    fn next(&mut self) -> Option<&Operator>;

    /// Get the current location
    fn curr_loc(&self) -> Location;

    /// Get the current instruction
    fn curr_op(&self) -> Option<&Operator>;
}
