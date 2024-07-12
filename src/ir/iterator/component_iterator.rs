use crate::ir::iterator::module_iterator::ModuleIterator;
use crate::ir::types::Location;
use std::collections::HashMap;

pub struct ComponentIterator {
    curr_mod: usize,
    num_mods: usize,
    mod_iterator: ModuleIterator,
    metadata: HashMap<usize, HashMap<usize, usize>>,
}

impl ComponentIterator {
    pub fn new(
        curr_mod: usize,
        num_mods: usize,
        metadata: HashMap<usize, HashMap<usize, usize>>,
    ) -> Self {
        // initializes to the first module
        ComponentIterator {
            curr_mod,
            num_mods,
            metadata: metadata.clone(),
            mod_iterator: ModuleIterator::new(
                metadata.get(&0).unwrap().keys().len(),
                (*metadata.get(&curr_mod).unwrap()).clone(),
            ),
        }
    }

    pub fn reset(&mut self) {
        self.curr_mod = 0;
        self.mod_iterator
            .reset_from_comp_iterator((*self.metadata.get(&self.curr_mod).unwrap()).clone());
    }

    fn next_module(&mut self) -> bool {
        self.curr_mod += 1;
        if self.curr_mod < self.num_mods {
            // If we're defining a new module, we have to reset function
            self.mod_iterator = ModuleIterator::new(
                self.metadata.get(&self.curr_mod).unwrap().keys().len(),
                self.metadata.get(&self.curr_mod).unwrap().clone(),
            );
            true
        } else {
            false
        }
    }

    pub fn curr_mod_idx(&self) -> usize {
        self.curr_mod
    }

    pub fn curr_func_idx(&self) -> usize {
        self.mod_iterator.curr_func
    }

    pub fn curr_instr_idx(&self) -> usize {
        self.mod_iterator.func_iterator.curr_instr
    }

    pub fn end(&self) -> bool {
        self.curr_mod == self.num_mods
    }

    pub fn curr_loc(&self) -> Location {
        if let Location::Module {
            func_idx,
            instr_idx,
        } = self.mod_iterator.curr_loc()
        {
            Location::Component {
                mod_idx: self.curr_mod,
                func_idx,
                instr_idx,
            }
        } else {
            panic!("Should have gotten Module Location from Module Iterator and not Component Location!");
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> bool {
        if self.mod_iterator.has_next() {
            self.mod_iterator.next()
        } else if self.next_module() {
            true
        } else {
            false
        }
    }
}
