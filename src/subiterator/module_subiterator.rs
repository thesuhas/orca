//! SubIterator for a Module

use crate::ir::id::FunctionID;
use crate::ir::types::Location;
use crate::subiterator::function_subiterator::FuncSubIterator;
use std::collections::HashMap;

/// Sub-iterator for a Module. Keeps track of current location in a Module.
pub struct ModuleSubIterator {
    /// The current function the SubIterator is at
    pub(crate) curr_func: FunctionID,
    /// The number of functions that have been visited thus far
    visited_funcs: usize,
    /// Number of functions in this module
    num_funcs: usize,
    /// Metadata that maps Function Index -> Instruction Index
    metadata: HashMap<FunctionID, usize>,
    /// The function iterator used to keep track of the location in the function.
    pub(crate) func_iterator: FuncSubIterator,
    /// Functions to skip. Provide an empty vector if no functions are to be skipped.
    skip_funcs: Vec<FunctionID>,
}

impl ModuleSubIterator {
    /// Creates a new ModuleSubIterator
    pub fn new(
        num_funcs: usize,
        metadata: HashMap<FunctionID, usize>,
        skip_funcs: Vec<FunctionID>,
    ) -> Self {
        let mut mod_it = ModuleSubIterator {
            curr_func: *metadata.keys().min().unwrap(),
            visited_funcs: 0,
            num_funcs,
            metadata: metadata.clone(),
            func_iterator: FuncSubIterator::new(
                *metadata.get(metadata.keys().min().unwrap()).unwrap(),
            ),
            skip_funcs,
        };
        // In case 0 is in skip func
        while mod_it
            .skip_funcs
            .contains(&(mod_it.curr_func as FunctionID))
        {
            mod_it.next_function();
        }
        mod_it
    }

    /// Checks if the SubIterator has finished traversing all the functions
    pub fn end(&self) -> bool {
        self.visited_funcs == self.num_funcs
    }

    /// Returns the current Location in the Module as a Location
    /// and a bool value that says whether the location is at the end of the function.
    pub fn curr_loc(&self) -> (Location, bool) {
        let curr_instr = self.func_iterator.curr_instr;
        (
            Location::Module {
                func_idx: self.curr_func,
                instr_idx: curr_instr,
            },
            self.func_iterator.is_end(curr_instr),
        )
    }

    /// Resets the ModuleSubIterator when it is a Child SubIterator of a ComponentSubIterator
    pub(crate) fn reset_from_comp_iterator(&mut self, metadata: HashMap<FunctionID, usize>) {
        self.curr_func = 0;
        self.metadata = metadata;
        self.func_iterator.reset(
            *self
                .metadata
                .get(self.metadata.keys().min().unwrap())
                .unwrap(),
        );
    }

    /// Resets the ModuleSubIterator when it is not a Child SubIterator
    pub fn reset(&mut self) {
        self.curr_func = 0;
        self.func_iterator.reset(*self.metadata.get(&0).unwrap());
    }

    /// Checks if there are functions left to visit
    pub fn has_next_function(&self) -> bool {
        self.curr_func + 1 < self.num_funcs as u32
    }

    /// Goes to the next function in the module
    fn next_function(&mut self) -> bool {
        self.curr_func += 1;
        self.visited_funcs += 1;

        // skip over configured funcs
        while self.visited_funcs < self.num_funcs
            && self.skip_funcs.contains(&(self.curr_func as FunctionID))
        {
            self.curr_func += 1;
            self.visited_funcs += 1;
        }
        if self.visited_funcs < self.num_funcs {
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
