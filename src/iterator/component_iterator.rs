//! Iterator to traverse a Component

use crate::ir::component::Component;
use crate::ir::types::{InstrumentType, Location};
use crate::iterator::iterator_trait::Iterator;
use crate::opcode::Opcode;
use crate::subiterator::component_subiterator::ComponentSubIterator;
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
    pub fn new(comp: &'a mut Component<'b>) -> Self {
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
            comp_iterator: ComponentSubIterator::new(0, num_modules, metadata),
        }
    }
}

impl<'a, 'b> Opcode<'b> for ComponentIterator<'a, 'b> {
    /// Injects an Operator at the current location
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
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = self.curr_loc()
        {
            self.comp.modules[mod_idx].code_sections[func_idx].instructions[instr_idx].1 =
                InstrumentType::InstrumentBefore(vec![]);
            self
        } else {
            panic!("Should have gotten component location!")
        }
    }

    /// Marks the current location as InstrumentAfter
    fn after(&mut self) -> &mut Self {
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = self.curr_loc()
        {
            self.comp.modules[mod_idx].code_sections[func_idx].instructions[instr_idx].1 =
                InstrumentType::InstrumentAfter(vec![]);
            self
        } else {
            panic!("Should have gotten component location!")
        }
    }

    /// Marks the current location as InstrumentAlternate
    fn alternate(&mut self) -> &mut Self {
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = self.curr_loc()
        {
            self.comp.modules[mod_idx].code_sections[func_idx].instructions[instr_idx].1 =
                InstrumentType::InstrumentAlternate(vec![]);
            self
        } else {
            panic!("Should have gotten component location!")
        }
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
    fn curr_instrument_type(&self) -> &InstrumentType {
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = self.comp_iterator.curr_loc()
        {
            &self.comp.modules[mod_idx].code_sections[func_idx].instructions[instr_idx].1
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
}
