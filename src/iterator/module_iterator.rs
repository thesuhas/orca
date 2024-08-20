//! Iterator to traverse a Module

use crate::ir::id::{FunctionID, GlobalID, LocalID};
use crate::ir::module::module_functions::{FuncKind, LocalFunction};
use crate::ir::module::module_globals::Global;
use crate::ir::module::Module;
use crate::ir::types::{DataType, InstrumentationMode, Location};
use crate::iterator::iterator_trait::{Instrumenter, Iterator};
use crate::module_builder::AddLocal;
use crate::opcode::{Inject, MacroOpcode, Opcode};
use crate::subiterator::module_subiterator::ModuleSubIterator;
use std::collections::HashMap;
use wasmparser::Operator;

/// Iterator for a Module.
// 'b should outlive 'a
pub struct ModuleIterator<'a, 'b> {
    /// The Module to Iterate
    pub module: &'a mut Module<'b>,
    /// The SubIterator for this Module
    mod_iterator: ModuleSubIterator,
}

#[allow(dead_code)]
impl<'a, 'b> ModuleIterator<'a, 'b> {
    /// Creates a new ModuleIterator
    pub fn new(module: &'a mut Module<'b>, skip_funcs: &Vec<FunctionID>) -> Self {
        // Creates Function -> Number of Instructions
        let mut metadata = HashMap::new();
        for func in module.functions.iter() {
            match &func.kind {
                FuncKind::Import(_) => {}
                FuncKind::Local(LocalFunction { func_id, body, .. }) => {
                    metadata.insert(*func_id, body.num_instructions);
                }
            }
        }
        let num_funcs = module.num_functions;
        ModuleIterator {
            module,
            mod_iterator: ModuleSubIterator::new(num_funcs, metadata, skip_funcs.to_owned()),
        }
    }

    pub fn curr_op_owned(&self) -> Option<Operator<'b>> {
        if self.mod_iterator.end() {
            None
        } else if let Location::Module {
            func_idx,
            instr_idx,
        } = self.mod_iterator.curr_loc()
        {
            match &self.module.functions.get(func_idx as FunctionID).kind {
                FuncKind::Import(_) => panic!("Cannot get an instruction to an imported function"),
                FuncKind::Local(l) => Some(l.body.instructions[instr_idx].0.clone()),
            }
        } else {
            panic!("Should have gotten Module Location!")
        }
    }
}

impl<'a, 'b> Inject<'b> for ModuleIterator<'a, 'b> {
    /// Injects an Operator at the current location
    ///
    /// # Example
    /// ```no_run
    /// use orca::ir::module::Module;
    /// use orca::iterator::module_iterator::ModuleIterator;
    /// use wasmparser::Operator;
    /// use orca::ir::types::{Location};
    /// use orca::iterator::iterator_trait::{Instrumenter, Iterator};
    /// use orca::opcode::Opcode;
    ///
    /// let file = "path_to_file";
    /// let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    /// // Must use `parse_only_module` here as we are only concerned about a Module and not a module that is inside a Component
    /// let mut module = Module::parse(&buff, false).expect("Unable to parse");
    /// let mut module_it = ModuleIterator::new(&mut module, &vec![]);
    ///
    /// // Everytime there is a `call 1` instruction we want to inject an `i32.const 0`
    /// let interested = Operator::Call { function_index: 1 };
    ///
    /// loop {
    ///     let op = module_it.curr_op();
    ///     let instr_mode = module_it.curr_instrument_mode();
    ///
    ///     if let Location::Module {
    ///         func_idx,
    ///         instr_idx,
    ///     } = module_it.curr_loc()
    ///     {
    ///         if *module_it.curr_op().unwrap() == interested {
    ///             module_it.before().i32_const(1);
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
            match self.module.functions.get_mut(func_idx as FunctionID).kind {
                FuncKind::Import(_) => panic!("Cannot get an instruction to an imported function"),
                FuncKind::Local(ref mut l) => l.body.instructions[instr_idx].1.add_instr(instr),
            }
        } else {
            panic!("Should have gotten Module Location!")
        }
    }
}
impl<'a, 'b> Opcode<'b> for ModuleIterator<'a, 'b> {}
impl<'a, 'b> MacroOpcode<'b> for ModuleIterator<'a, 'b> {}
impl<'a, 'b> Instrumenter<'b> for ModuleIterator<'a, 'b> {
    /// Returns the Instrumentation at the current Location
    fn curr_instrument_mode(&self) -> &Option<InstrumentationMode> {
        if let Location::Module {
            func_idx,
            instr_idx,
        } = self.mod_iterator.curr_loc()
        {
            match &self.module.functions.get(func_idx as FunctionID).kind {
                FuncKind::Import(_) => panic!("Cannot get an instruction to an imported function"),
                FuncKind::Local(l) => &l.body.instructions[instr_idx].1.current_mode,
            }
        } else {
            panic!("Should have gotten Module Location and not Module Location!")
        }
    }

    fn set_instrument_mode(&mut self, mode: InstrumentationMode) {
        if let Location::Module { .. } = self.curr_loc() {
            self.set_instrument_mode_at(mode, self.curr_loc());
        } else {
            panic!("Should have gotten module location!")
        }
    }

    fn set_instrument_mode_at(&mut self, mode: InstrumentationMode, loc: Location) {
        if let Location::Module {
            func_idx,
            instr_idx,
        } = loc
        {
            match self.module.functions.get_mut(func_idx as FunctionID).kind {
                FuncKind::Import(_) => panic!("Cannot add an instruction to an imported function"),
                FuncKind::Local(ref mut l) => {
                    l.body.instructions[instr_idx].1.current_mode = Some(mode)
                }
            }
        } else {
            panic!("Should have gotten module location!")
        }
    }

    fn add_instr_at(&mut self, loc: Location, instr: Operator<'b>) {
        if let Location::Module {
            func_idx,
            instr_idx,
        } = loc
        {
            match self.module.functions.get_mut(func_idx as FunctionID).kind {
                FuncKind::Import(_) => panic!("Cannot add an instruction to an imported function"),
                FuncKind::Local(ref mut l) => {
                    let instr_of_loc = &mut l.body.instructions[instr_idx].1;
                    instr_of_loc.add_instr(instr);
                }
            }
            // Only injects if it is an instrumented location
        } else {
            panic!("Should have gotten Module Location!")
        }
    }

    fn before_at(&mut self, loc: Location) -> &mut Self {
        if let Location::Module { .. } = loc {
            self.set_instrument_mode_at(InstrumentationMode::Before, loc);
            self
        } else {
            panic!("Should have gotten Module Location!")
        }
    }

    fn after_at(&mut self, loc: Location) -> &mut Self {
        if let Location::Module { .. } = loc {
            self.set_instrument_mode_at(InstrumentationMode::After, loc);
            self
        } else {
            panic!("Should have gotten Module Location!")
        }
    }

    fn alternate_at(&mut self, loc: Location) -> &mut Self {
        if let Location::Module { .. } = loc {
            self.set_instrument_mode_at(InstrumentationMode::Alternate, loc);
            self
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
            match &self.module.functions.get(func_idx as FunctionID).kind {
                FuncKind::Import(_) => panic!("Cannot get an instruction to an imported function"),
                FuncKind::Local(l) => l.body.instructions[instr_idx].1.get_instr(idx),
            }
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }
    }

    fn add_global(&mut self, global: Global) -> GlobalID {
        self.module.globals.add(global)
    }
}

impl<'a, 'b> AddLocal for ModuleIterator<'a, 'b> {
    fn add_local(&mut self, val_type: DataType) -> LocalID {
        let curr_loc = self.curr_loc();
        if let Location::Module {
            func_idx,
            instr_idx: _,
        } = curr_loc
        {
            self.module.functions.add_local(func_idx, val_type)
        } else {
            panic!("Should have gotten Module Location!")
        }
    }
}

// Note: Marked Trait as the same lifetime as component
impl<'a, 'b> Iterator<'b> for ModuleIterator<'a, 'b> {
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

    /// Returns the current instruction
    fn curr_op(&self) -> Option<&Operator<'b>> {
        if self.mod_iterator.end() {
            None
        } else if let Location::Module {
            func_idx,
            instr_idx,
        } = self.mod_iterator.curr_loc()
        {
            match &self.module.functions.get(func_idx as FunctionID).kind {
                FuncKind::Import(_) => panic!("Cannot get an instruction to an imported function"),
                FuncKind::Local(l) => Some(&l.body.instructions[instr_idx].0),
            }
        } else {
            panic!("Should have gotten Module Location!")
        }
    }
}
