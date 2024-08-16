//! Iterator to traverse a Component

use crate::ir::component::Component;
use crate::ir::id::{FunctionID, GlobalID, LocalID, ModuleID};
use crate::ir::module::module_functions::FuncKind;
use crate::ir::module::module_globals::Global;
use crate::ir::types::{DataType, InstrumentationMode, Location};
use crate::iterator::iterator_trait::{Instrumenter, Iterator};
use crate::opcode::{Inject, MacroOpcode, Opcode};
use crate::subiterator::component_subiterator::ComponentSubIterator;
use crate::ModuleBuilder;
use std::collections::HashMap;
use std::iter::Iterator as StdIter;
use wasmparser::Operator;

/// Iterator for a Component.
pub struct ComponentIterator<'a, 'b> {
    /// The Component to iterate
    pub comp: &'a mut Component<'b>,
    /// The SubIterator for this Component
    comp_iterator: ComponentSubIterator,
}

fn print_metadata(metadata: &HashMap<usize, HashMap<usize, usize>>) {
    for c in metadata.keys() {
        println!("Module: {:?}", c);
        for (m, i) in metadata.get(c).unwrap().iter() {
            println!("Function: {:?} Instr: {:?}", m, i);
        }
    }
}

#[allow(dead_code)]
impl<'a, 'b> ComponentIterator<'a, 'b> {
    /// Creates a new Component Iterator
    pub fn new(comp: &'a mut Component<'b>, skip_funcs: HashMap<usize, Vec<usize>>) -> Self {
        // Creates Module -> Function -> Number of Instructions
        let mut metadata = HashMap::new();
        for (mod_idx, m) in comp.modules.iter().enumerate() {
            let mut mod_metadata = HashMap::new();
            for (idx, func) in m.functions.iter().enumerate() {
                match &func.kind {
                    FuncKind::Import(_) => {}
                    FuncKind::Local(l) => {
                        mod_metadata.insert(idx, l.body.num_instructions);
                    }
                }
            }
            metadata.insert(mod_idx, mod_metadata);
        }
        print_metadata(&metadata);
        let num_modules = comp.num_modules;
        ComponentIterator {
            comp,
            comp_iterator: ComponentSubIterator::new(0, num_modules, metadata, skip_funcs),
        }
    }

    /// Returns the current module the component iterator is in
    pub fn curr_module(&self) -> ModuleID {
        if let Location::Component {
            mod_idx,
            func_idx: _func_idx,
            instr_idx: _instr_idx,
        } = self.curr_loc()
        {
            mod_idx as u32
        } else {
            panic!("Should have gotten component location");
        }
    }

    pub fn curr_op_owned(&self) -> Option<Operator<'b>> {
        if self.comp_iterator.end() {
            None
        } else if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = self.comp_iterator.curr_loc()
        {
            match &self.comp.modules[mod_idx]
                .functions
                .get(func_idx as FunctionID)
                .kind
            {
                FuncKind::Import(_) => None,
                FuncKind::Local(l) => Some(l.body.instructions[instr_idx].0.clone()),
            }
        } else {
            panic!("Should have gotten Component Location!")
        }
    }

    pub fn add_local(&mut self, val_type: DataType) -> LocalID {
        let curr_loc = self.curr_loc();
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx: _,
        } = curr_loc
        {
            {
                self.comp.modules[mod_idx]
                    .functions
                    .add_local(func_idx as FunctionID, val_type)
            }
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }
    }
}

impl<'a, 'b> Inject<'b> for ComponentIterator<'a, 'b> {
    /// Injects an Operator at the current location
    ///
    /// # Example
    /// ```no_run
    /// use std::collections::HashMap;
    /// use orca::ir::component::Component;
    /// use orca::iterator::component_iterator::ComponentIterator;
    /// use wasmparser::Operator;
    /// use orca::ir::types::{Location};
    /// use orca::iterator::iterator_trait::{Instrumenter, Iterator};
    /// use orca::opcode::Opcode;
    ///
    /// let file = "path_to_file";
    /// let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    /// let mut component = Component::parse(&buff, false).expect("Unable to parse");
    /// let mut comp_it = ComponentIterator::new(&mut component, HashMap::new());
    ///
    /// // Everytime there is a `call 1` instruction we want to inject an `i32.const 0`
    /// let interested = Operator::Call { function_index: 1 };
    ///
    /// loop {
    ///     let op = comp_it.curr_op();
    ///     let instr_mode = comp_it.curr_instrument_mode();
    ///
    ///     if let Location::Component {
    ///         mod_idx,
    ///         func_idx,
    ///         instr_idx,
    ///     } = comp_it.curr_loc()
    ///     {
    ///         if *comp_it.curr_op().unwrap() == interested {
    ///             comp_it.before().i32_const(1);
    ///         }
    ///         if comp_it.next().is_none() {
    ///             break;
    ///         };
    ///     } else {
    ///         // Ensures we only get the location of a module while parsing a component
    ///         panic!("Should've gotten Component Location!");
    ///     }
    /// }
    /// ```
    fn inject(&mut self, instr: Operator<'b>) {
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = self.curr_loc()
        {
            match self.comp.modules[mod_idx]
                .functions
                .get_mut(func_idx as FunctionID)
                .kind
            {
                FuncKind::Import(_) => panic!("Can't inject into an imported function!"),
                FuncKind::Local(ref mut l) => l.body.instructions[instr_idx].1.add_instr(instr),
            }
        } else {
            panic!("Should have gotten component location!")
        }
    }
}
impl<'a, 'b> Opcode<'b> for ComponentIterator<'a, 'b> {}
impl<'a, 'b> MacroOpcode<'b> for ComponentIterator<'a, 'b> {}
impl<'a, 'b> Instrumenter<'b> for ComponentIterator<'a, 'b> {
    /// Returns the Instrumentation at the current Location
    fn curr_instrument_mode(&self) -> &Option<InstrumentationMode> {
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = self.comp_iterator.curr_loc()
        {
            match &self.comp.modules[mod_idx]
                .functions
                .get(func_idx as FunctionID)
                .kind
            {
                FuncKind::Import(_) => {
                    panic!("Can't get instrumentation from an imported function!")
                }
                FuncKind::Local(l) => &l.body.instructions[instr_idx].1.current_mode,
            }
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }
    }

    /// Sets the type of Instrumentation Mode of the current location
    fn set_instrument_mode(&mut self, mode: InstrumentationMode) {
        if let Location::Component {
            mod_idx: _mod_idx,
            func_idx: _func_idx,
            instr_idx: _instr_idx,
        } = self.curr_loc()
        {
            self.set_instrument_mode_at(mode, self.curr_loc());
        } else {
            panic!("Should have gotten component location!")
        }
    }

    fn set_instrument_mode_at(&mut self, mode: InstrumentationMode, loc: Location) {
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = loc
        {
            match self.comp.modules[mod_idx]
                .functions
                .get_mut(func_idx as FunctionID)
                .kind
            {
                FuncKind::Import(_) => panic!("Can't instrument into an imported function!"),
                FuncKind::Local(ref mut l) => {
                    l.body.instructions[instr_idx].1.current_mode = Some(mode)
                }
            }
        } else {
            panic!("Should have gotten component location!")
        }
    }

    fn add_instr_at(&mut self, loc: Location, instr: Operator<'b>) {
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = loc
        {
            match self.comp.modules[mod_idx]
                .functions
                .get_mut(func_idx as FunctionID)
                .kind
            {
                FuncKind::Import(_) => panic!("Can't instrument into an imported function!"),
                FuncKind::Local(ref mut l) => {
                    // Only injects if it is an instrumented location
                    let instr_of_loc = &mut l.body.instructions[instr_idx].1;
                    instr_of_loc.add_instr(instr);
                }
            }
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }
    }

    fn before_at(&mut self, loc: Location) -> &mut Self {
        if let Location::Component { .. } = loc {
            self.set_instrument_mode_at(InstrumentationMode::Before, loc);
            self
        } else {
            panic!("Should have gotten Component location!");
        }
    }

    fn after_at(&mut self, loc: Location) -> &mut Self {
        if let Location::Component { .. } = loc {
            self.set_instrument_mode_at(InstrumentationMode::After, loc);
            self
        } else {
            panic!("Should have gotten Component location!");
        }
    }

    fn alternate_at(&mut self, loc: Location) -> &mut Self {
        if let Location::Component {
            mod_idx: _mod_idx,
            func_idx: _func_idx,
            instr_idx: _instr_idx,
        } = loc
        {
            self.set_instrument_mode_at(InstrumentationMode::Alternate, loc);
            self
        } else {
            panic!("Should have gotten Component location!");
        }
    }

    /// Gets the injected instruction at the current location by index
    fn get_injected_val(&self, idx: usize) -> &Operator {
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = self.comp_iterator.curr_loc()
        {
            match &self.comp.modules[mod_idx]
                .functions
                .get(func_idx as FunctionID)
                .kind
            {
                FuncKind::Import(_) => panic!("Can't inject into an imported function!"),
                FuncKind::Local(l) => l.body.instructions[instr_idx].1.get_instr(idx),
            }
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }
    }

    fn add_global(&mut self, global: Global) -> GlobalID {
        let curr_mod = self.curr_module() as usize;
        self.comp.modules[curr_mod].globals.add(global)
    }
}

// Note: Marked Trait as the same lifetime as component
impl<'a, 'b> Iterator<'b> for ComponentIterator<'a, 'b> {
    /// Resets the Component Iterator
    fn reset(&mut self) {
        self.comp_iterator.reset();
    }

    /// Goes to the next instruction
    fn next(&mut self) -> Option<&Operator> {
        match self.comp_iterator.next() {
            false => None,
            true => self.curr_op(),
        }
    }

    /// Returns the current location
    fn curr_loc(&self) -> Location {
        self.comp_iterator.curr_loc()
    }

    /// Returns the instruction at the current location
    fn curr_op(&self) -> Option<&Operator> {
        if self.comp_iterator.end() {
            None
        } else if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = self.comp_iterator.curr_loc()
        {
            match &self.comp.modules[mod_idx]
                .functions
                .get(func_idx as FunctionID)
                .kind
            {
                FuncKind::Import(_) => panic!("Can't inject into an imported function!"),
                FuncKind::Local(l) => Some(&l.body.instructions[instr_idx].0),
            }
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }
    }
}

impl ModuleBuilder for ComponentIterator<'_, '_> {
    fn add_local(&mut self, val_type: DataType) -> LocalID {
        let curr_loc = self.curr_loc();
        if let Location::Component {
            mod_idx, func_idx, ..
        } = curr_loc
        {
            {
                self.comp.modules[mod_idx]
                    .functions
                    .add_local(func_idx as FunctionID, val_type)
            }
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }
    }
}
