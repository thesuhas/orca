//! Iterator to traverse a Component

use crate::ir::component::Component;
use crate::ir::id::LocalID;
use crate::ir::types::{Instrument, InstrumentType, InstrumentationMode, Location};
use crate::iterator::iterator_trait::Iterator;
use crate::opcode::Opcode;
use crate::subiterator::component_subiterator::ComponentSubIterator;
use crate::ModuleBuilder;
use std::collections::HashMap;
use wasmparser::Operator;

/// Iterator for a Component.
pub struct ComponentIterator<'a, 'b> {
    /// The Component to iterate
    comp: &'a mut Component<'b>,
    /// The SubIterator for this Component
    comp_iterator: ComponentSubIterator,
}

#[allow(dead_code)]
impl<'a, 'b> ComponentIterator<'a, 'b> {
    /// Creates a new Component Iterator
    pub fn new(comp: &'a mut Component<'b>, skip_funcs: HashMap<usize, Vec<usize>>) -> Self {
        // Creates Module -> Function -> Number of Instructions
        let mut metadata = HashMap::new();
        for (mod_idx, m) in comp.modules.iter().enumerate() {
            let mut mod_metadata = HashMap::new();
            for (idx, func) in m.code_sections.iter().enumerate() {
                mod_metadata.insert(idx, func.num_instructions);
            }
            metadata.insert(mod_idx, mod_metadata);
        }
        let num_modules = comp.num_modules;
        ComponentIterator {
            comp,
            comp_iterator: ComponentSubIterator::new(0, num_modules, metadata, skip_funcs),
        }
    }
}

impl<'a, 'b> Opcode<'b> for ComponentIterator<'a, 'b> {
    /// Injects an Operator at the current location
    ///
    /// # Example
    /// ```no_run
    /// use std::collections::HashMap;
    /// use orca::ir::component::Component;
    /// use orca::iterator::component_iterator::ComponentIterator;
    /// use wasmparser::Operator;
    /// use orca::ir::types::{Location};
    /// use orca::iterator::iterator_trait::Iterator;
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
    ///     let instr_type = comp_it.curr_instrument_type();
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
            self.comp.modules[mod_idx].code_sections[func_idx].instructions[instr_idx]
                .1
                .add_instr(instr)
        } else {
            panic!("Should have gotten component location!")
        }
    }
}

// Note: Marked Trait as the same lifetime as component
impl<'a, 'b> Iterator<'b> for ComponentIterator<'a, 'b> {
    /// Marks the current location as InstrumentBefore
    fn before(&mut self) -> &mut Self {
        self.set_instrument_type(InstrumentationMode::Before);
        self
    }

    /// Marks the current location as InstrumentAfter
    fn after(&mut self) -> &mut Self {
        self.set_instrument_type(InstrumentationMode::After);
        self
    }

    /// Marks the current location as InstrumentAlternate
    fn alternate(&mut self) -> &mut Self {
        self.set_instrument_type(InstrumentationMode::Alternate);
        self
    }

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

    /// Returns the Instrumentation at the current Location
    fn curr_instrument_type(&self) -> InstrumentType {
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = self.comp_iterator.curr_loc()
        {
            self.comp.modules[mod_idx].code_sections[func_idx].instructions[instr_idx]
                .1
                .get_curr()
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }
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
            Some(&self.comp.modules[mod_idx].code_sections[func_idx].instructions[instr_idx].0)
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
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
            self.comp.modules[mod_idx].code_sections[func_idx].instructions[instr_idx]
                .1
                .get_instr(idx)
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }
    }

    /// Sets the type of Instrumentation Mode of the current location
    fn set_instrument_type(&mut self, mode: InstrumentationMode) {
        if let Location::Component {
            mod_idx: _mod_idx,
            func_idx: _func_idx,
            instr_idx: _instr_idx,
        } = self.curr_loc()
        {
            self.set_instrument_type_at(mode, self.curr_loc());
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
            // Only injects if it is an instrumented location
            let instr_of_loc =
                &mut self.comp.modules[mod_idx].code_sections[func_idx].instructions[instr_idx].1;
            match instr_of_loc {
                Instrument::NotInstrumented => {
                    panic!("Can't inject to a location that is not instrumented!")
                }
                Instrument::Instrumented {
                    before: _before,
                    after: _after,
                    alternate: _alternate,
                    current: _current,
                } => instr_of_loc.add_instr(instr),
            }
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }
    }

    fn before_at(&mut self, loc: Location) -> &mut Self {
        if let Location::Component {
            mod_idx: _mod_idx,
            func_idx: _func_idx,
            instr_idx: _instr_idx,
        } = loc
        {
            self.set_instrument_type_at(InstrumentationMode::Before, loc);
            self
        } else {
            panic!("Should have gotten Component location!");
        }
    }

    fn after_at(&mut self, loc: Location) -> &mut Self {
        if let Location::Component {
            mod_idx: _mod_idx,
            func_idx: _func_idx,
            instr_idx: _instr_idx,
        } = loc
        {
            self.set_instrument_type_at(InstrumentationMode::After, loc);
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
            self.set_instrument_type_at(InstrumentationMode::Alternate, loc);
            self
        } else {
            panic!("Should have gotten Component location!");
        }
    }

    fn set_instrument_type_at(&mut self, mode: InstrumentationMode, loc: Location) {
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = loc
        {
            if self.comp.modules[mod_idx].code_sections[func_idx].instructions[instr_idx]
                .1
                .get_curr()
                == InstrumentType::NotInstrumented
            {
                self.comp.modules[mod_idx].code_sections[func_idx].instructions[instr_idx].1 =
                    Instrument::Instrumented {
                        before: vec![],
                        after: vec![],
                        alternate: vec![],
                        current: mode,
                    }
            } else {
                self.comp.modules[mod_idx].code_sections[func_idx].instructions[instr_idx]
                    .1
                    .set_curr(mode);
            }
        } else {
            panic!("Should have gotten component location!")
        }
    }
}

impl ModuleBuilder for ComponentIterator<'_, '_> {
    fn add_local(&mut self, _val_type: crate::ir::types::DataType) -> LocalID {
        let curr_loc = self.curr_loc();
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx: _,
        } = curr_loc
        {
            self.comp.modules[mod_idx].add_local(func_idx, _val_type)
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }
    }
}
