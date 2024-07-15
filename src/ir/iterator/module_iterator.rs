//! Iterator to traverse a Module

use crate::ir::iterator::iterator_trait::Iterator;
use crate::ir::module::Module;
use crate::ir::subiterator::module_subiterator::ModuleSubIterator;
use crate::ir::types::{InstrumentType, Location};
use std::collections::HashMap;
use wasmparser::Operator;

/// Iterator for a Module.
pub struct ModuleIterator<'a, 'b> {
    /// The Module to Iterate
    module: &'a mut Module<'b>,
    /// The SubIterator for this Module
    mod_iterator: ModuleSubIterator,
}

#[allow(dead_code)]
impl<'a, 'b> ModuleIterator<'a, 'b> {
    /// Creates a new ModuleIterator
    pub fn new(module: &'a mut Module<'b>) -> Self {
        // Creates Function -> Number of Instructions
        let mut metadata = HashMap::new();
        for (idx, func) in module.code_sections.iter().enumerate() {
            metadata.insert(idx, func.num_instructions);
        }
        let num_funcs = module.num_functions;
        ModuleIterator {
            module,
            mod_iterator: ModuleSubIterator::new(num_funcs, metadata),
        }
    }
}

// Note: Marked Trait as the same lifetime as component
impl<'a, 'b> Iterator<'b> for ModuleIterator<'a, 'b> {
    /// Injects an Operator into the current location
    fn inject(&mut self, instr: Operator<'b>) {
        if let Location::Module {
            func_idx,
            instr_idx,
        } = self.curr_loc()
        {
            self.module.code_sections[func_idx].instructions[instr_idx]
                .1
                .add_instr(instr)
        } else {
            panic!("Should have gotten Module Location!")
        }
    }

    /// Marks the current location as InstrumentBefore
    fn before(&mut self) -> &mut Self {
        if let Location::Module {
            func_idx,
            instr_idx,
        } = self.curr_loc()
        {
            self.module.code_sections[func_idx].instructions[instr_idx].1 =
                InstrumentType::InstrumentBefore(vec![]);
            self
        } else {
            panic!("Should have gotten Module Location!")
        }
    }

    /// Marks the current location as InstrumentAfter
    fn after(&mut self) -> &mut Self {
        if let Location::Module {
            func_idx,
            instr_idx,
        } = self.curr_loc()
        {
            self.module.code_sections[func_idx].instructions[instr_idx].1 =
                InstrumentType::InstrumentAfter(vec![]);
            self
        } else {
            panic!("Should have gotten Module Location!")
        }
    }

    /// Marks the current location as InstrumentAlternate
    fn alternate(&mut self) -> &mut Self {
        if let Location::Module {
            func_idx,
            instr_idx,
        } = self.curr_loc()
        {
            self.module.code_sections[func_idx].instructions[instr_idx].1 =
                InstrumentType::InstrumentAlternate(vec![]);
            self
        } else {
            panic!("Should have gotten Module Location!")
        }
    }

    /// Resets the Module Iterator
    fn reset(&mut self) {
        self.mod_iterator.reset();
    }

    /// Goes to the next instruction and returns the instruction
    fn next(&mut self) -> Option<&Operator> {
        match self.mod_iterator.next() {
            false => None,
            true => self.curr_op(),
        }
    }

    /// Returns the current Location
    fn curr_loc(&self) -> Location {
        self.mod_iterator.curr_loc()
    }

    /// Returns the Instrumentation at the current Location
    fn curr_instrument_type(&self) -> &InstrumentType {
        if let Location::Module {
            func_idx,
            instr_idx,
        } = self.mod_iterator.curr_loc()
        {
            &self.module.code_sections[func_idx].instructions[instr_idx].1
        } else {
            panic!("Should have gotten Module Location and not Module Location!")
        }
    }

    /// Returns the current instruction
    fn curr_op(&self) -> Option<&Operator> {
        if self.mod_iterator.end() {
            None
        } else if let Location::Module {
            func_idx,
            instr_idx,
        } = self.mod_iterator.curr_loc()
        {
            Some(&self.module.code_sections[func_idx].instructions[instr_idx].0)
        } else {
            panic!("Should have gotten Module Location!")
        }
    }

    /// Gets the injected instruction at the current location by index
    fn get_injected_val(&self, idx: usize) -> &Operator {
        if let Location::Module {
            func_idx,
            instr_idx,
        } = self.mod_iterator.curr_loc()
        {
            self.module.code_sections[func_idx].instructions[instr_idx]
                .1
                .get_instr(idx)
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }
    }
}
