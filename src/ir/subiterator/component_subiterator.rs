use crate::ir::subiterator::module_subiterator::ModuleSubIterator;
use crate::ir::types::Location;
use std::collections::HashMap;

/// Sub-iterator for a Component. Keeps track of current location in a Component.
pub struct ComponentSubIterator {
    /// The current module the SubIterator is at.
    curr_mod: usize,
    /// The total number of modules in the component.
    num_mods: usize,
    /// The module iterator used to keep track of the location in the module.
    mod_iterator: ModuleSubIterator,
    /// Metadata that maps Module Index -> Function Index -> Instruction Index
    metadata: HashMap<usize, HashMap<usize, usize>>,
}

impl ComponentSubIterator {
    /// Creates a new ComponentSubIterator
    pub fn new(
        curr_mod: usize,
        num_mods: usize,
        metadata: HashMap<usize, HashMap<usize, usize>>,
    ) -> Self {
        // initializes to the first module
        ComponentSubIterator {
            curr_mod,
            num_mods,
            metadata: metadata.clone(),
            mod_iterator: ModuleSubIterator::new(
                metadata.get(&0).unwrap().keys().len(),
                (*metadata.get(&curr_mod).unwrap()).clone(),
            ),
        }
    }

    /// Resets the ComponentSubIterator and all child SubIterators
    pub fn reset(&mut self) {
        self.curr_mod = 0;
        self.mod_iterator
            .reset_from_comp_iterator((*self.metadata.get(&self.curr_mod).unwrap()).clone());
    }

    /// Goes to the next module enclosed by the component
    fn next_module(&mut self) -> bool {
        self.curr_mod += 1;
        if self.curr_mod < self.num_mods {
            // If we're defining a new module, we have to reset function
            self.mod_iterator = ModuleSubIterator::new(
                self.metadata.get(&self.curr_mod).unwrap().keys().len(),
                self.metadata.get(&self.curr_mod).unwrap().clone(),
            );
            true
        } else {
            false
        }
    }

    /// Gets the index of the current module in the component
    pub fn curr_mod_idx(&self) -> usize {
        self.curr_mod
    }

    /// Gets the index of the current function in the current module
    pub fn curr_func_idx(&self) -> usize {
        self.mod_iterator.curr_func
    }

    /// Gets the index of the current instruction in the current function
    pub fn curr_instr_idx(&self) -> usize {
        self.mod_iterator.func_iterator.curr_instr
    }

    /// Checks if the SubIterator has finished traversing all the modules
    pub fn end(&self) -> bool {
        self.curr_mod == self.num_mods
    }

    /// Returns the Current Location as a Location
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
    /// Goes to the next instruction in the component
    pub fn next(&mut self) -> bool {
        if self.mod_iterator.has_next() {
            self.mod_iterator.next()
        } else {
            self.next_module()
        }
    }
}
