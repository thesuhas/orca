//! Iterator to traverse a Module

use crate::ir::module::Module;
use crate::ir::types::{InstrumentType, InstrumentationMode, Location};
use crate::iterator::iterator_trait::Iterator;
use crate::opcode::Opcode;
use crate::subiterator::module_subiterator::ModuleSubIterator;
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

impl<'a, 'b> Opcode<'b> for ModuleIterator<'a, 'b> {
    /// Injects an Operator at the current location
    ///
    /// # Example
    /// ```rust
    /// use orca::ir::module::Module;
    /// use orca::iterator::module_iterator::ModuleIterator;
    /// use wasmparser::Operator;
    /// use orca::ir::types::{Location};
    /// use orca::iterator::iterator_trait::Iterator;
    /// use orca::opcode::Opcode;
    ///
    /// let file = "path_to_file";
    /// let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    /// // Must use `parse_only_module` here as we are only concerned about a Module and not a module that is inside a Component
    /// let mut module = Module::parse_only_module(&buff, false).expect("Unable to parse");
    /// let mut module_it = ModuleIterator::new(&mut module);
    ///
    /// // Everytime there is a `call 1` instruction we want to inject an `i32.const 0`
    /// let interested = Operator::Call { function_index: 1 };
    ///
    /// loop {
    ///     let op = module_it.curr_op();
    ///     let instr_type = module_it.curr_instrument_type();
    ///
    ///     if let Location::Module {
    ///         func_idx,
    ///         instr_idx,
    ///     } = module_it.curr_loc()
    ///     {
    ///         if *module_it.curr_op().unwrap() == interested {
    ///             module_it.before().i32(1);
    ///         }
    ///         if module_it.next().is_none() {
    ///             break;
    ///         };
    ///     } else {
    ///         // Ensures we only get the location of a module while parsing a component
    ///         panic!("Should've gotten Module Location!");
    ///     }
    /// }
    /// ```
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
}

// Note: Marked Trait as the same lifetime as component
impl<'a, 'b> Iterator<'b> for ModuleIterator<'a, 'b> {
    /// Marks the current location as InstrumentBefore
    fn before(&mut self) -> &mut Self {
        if let Location::Module {
            func_idx,
            instr_idx,
        } = self.curr_loc()
        {
            self.module.code_sections[func_idx].instructions[instr_idx]
                .1
                .set_curr(InstrumentationMode::Before);
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
            self.module.code_sections[func_idx].instructions[instr_idx]
                .1
                .set_curr(InstrumentationMode::After);
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
            self.module.code_sections[func_idx].instructions[instr_idx]
                .1
                .set_curr(InstrumentationMode::Alternate);
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
    fn curr_instrument_type(&self) -> InstrumentType {
        if let Location::Module {
            func_idx,
            instr_idx,
        } = self.mod_iterator.curr_loc()
        {
            self.module.code_sections[func_idx].instructions[instr_idx]
                .1
                .get_curr()
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
