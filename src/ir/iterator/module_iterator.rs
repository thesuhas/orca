use crate::ir::iterator::function_iterator::FuncIterator;
use crate::ir::types::Location;
use std::collections::HashMap;

pub struct ModuleIterator {
    pub(crate) curr_func: usize,
    num_funcs: usize,
    // Function -> Number of Instructions
    metadata: HashMap<usize, usize>,
    pub(crate) func_iterator: FuncIterator,
}

impl ModuleIterator {
    pub fn new(num_funcs: usize, metadata: HashMap<usize, usize>) -> Self {
        ModuleIterator {
            curr_func: 0,
            num_funcs,
            metadata: metadata.clone(),
            func_iterator: FuncIterator::new(*metadata.get(&0).unwrap()),
        }
    }

    pub fn end(&self) -> bool {
        self.curr_func == self.num_funcs
    }

    pub fn curr_loc(&self) -> Location {
        Location::Module {
            func_idx: self.curr_func,
            instr_idx: self.func_iterator.curr_instr,
        }
    }

    pub(crate) fn reset_from_comp_iterator(&mut self, metadata: HashMap<usize, usize>) {
        self.curr_func = 0;
        self.metadata = metadata;
        self.func_iterator.reset(*self.metadata.get(&0).unwrap());
    }

    pub fn reset(&mut self) {
        self.curr_func = 0;
        self.func_iterator.reset(*self.metadata.get(&0).unwrap());
    }

    pub fn has_next_function(&self) -> bool {
        self.curr_func + 1 < self.num_funcs
    }

    fn next_function(&mut self) -> bool {
        self.curr_func += 1;
        if self.curr_func < self.num_funcs {
            self.func_iterator = FuncIterator::new(*self.metadata.get(&self.curr_func).unwrap());
            true
        } else {
            false
        }
    }

    pub(crate) fn has_next(&self) -> bool {
        self.func_iterator.has_next() || self.has_next_function()
    }

    pub(crate) fn next(&mut self) -> bool {
        if self.func_iterator.has_next() {
            self.func_iterator.next()
        } else if self.next_function() {
            true
        } else {
            false
        }
    }
}
