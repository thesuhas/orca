//! SubIterator for a Module

use crate::ir::types::Location;
use crate::subiterator::function_subiterator::FuncSubIterator;
use std::collections::HashMap;

/// Sub-iterator for a Module. Keeps track of current location in a Module.
pub struct ModuleSubIterator {
    /// The current function the SubIterator is at
    pub(crate) curr_func: usize,
    /// Number of functions in this module
    num_funcs: usize,
    /// Metadata that maps Function Index -> Instruction Index
    metadata: HashMap<usize, usize>,
    /// The function iterator used to keep track of the location in the function.
    pub(crate) func_iterator: FuncSubIterator,
}

impl ModuleSubIterator {
    /// Creates a new ModuleSubIterator
    pub fn new(num_funcs: usize, metadata: HashMap<usize, usize>) -> Self {
        ModuleSubIterator {
            curr_func: 0,
            num_funcs,
            metadata: metadata.clone(),
            func_iterator: FuncSubIterator::new(*metadata.get(&0).unwrap()),
        }
    }

    /// Checks if the SubIterator has finished traversing all the functions
    pub fn end(&self) -> bool {
        self.curr_func == self.num_funcs
    }

    /// Returns the current Location in the Module as a Location
    pub fn curr_loc(&self) -> Location {
        Location::Module {
            func_idx: self.curr_func,
            instr_idx: self.func_iterator.curr_instr,
        }
    }

    /// Resets the ModuleSubIterator when it is a Child SubIterator of a ComponentSubIterator
    pub(crate) fn reset_from_comp_iterator(&mut self, metadata: HashMap<usize, usize>) {
        self.curr_func = 0;
        self.metadata = metadata;
        self.func_iterator.reset(*self.metadata.get(&0).unwrap());
    }

    /// Resets the ModuleSubIterator when it is not a Child SubIterator
    pub fn reset(&mut self) {
        self.curr_func = 0;
        self.func_iterator.reset(*self.metadata.get(&0).unwrap());
    }

    /// Checks if there are functions left to visit
    pub fn has_next_function(&self) -> bool {
        self.curr_func + 1 < self.num_funcs
    }

    /// Goes to the next function in the module
    fn next_function(&mut self) -> bool {
        self.curr_func += 1;
        if self.curr_func < self.num_funcs {
            self.func_iterator = FuncSubIterator::new(*self.metadata.get(&self.curr_func).unwrap());
            true
        } else {
            false
        }
    }

    /// Checks if there are functions left to visit
    pub(crate) fn has_next(&self) -> bool {
        self.func_iterator.has_next() || self.has_next_function()
    }

    /// Goes to the next function
    pub(crate) fn next(&mut self) -> bool {
        if self.func_iterator.has_next() {
            self.func_iterator.next()
        } else {
            self.next_function()
        }
    }
}
