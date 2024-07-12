use crate::ir::component::Component;
use crate::ir::injector::injector_trait::Injector;
use crate::ir::iterator::component_iterator::ComponentIterator;
use crate::ir::types::{InstrumentType, Location};
use std::collections::HashMap;
use wasmparser::Operator;

pub struct ComponentInjector<'a, 'b> {
    comp: &'a mut Component<'b>,
    comp_iterator: ComponentIterator,
}

#[allow(dead_code)]
impl<'a, 'b> ComponentInjector<'a, 'b> {
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
        ComponentInjector {
            comp,
            comp_iterator: ComponentIterator::new(0, num_modules, metadata),
        }
    }
}

// Note: Marked Trait as the same lifetime as component
impl<'a, 'b> Injector<'b> for ComponentInjector<'a, 'b> {
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

    fn reset(&mut self) {
        self.comp_iterator.reset();
    }

    fn next(&mut self) -> Option<&Operator> {
        match self.comp_iterator.next() {
            false => None,
            true => self.curr_op(),
        }
    }

    fn curr_loc(&self) -> Location {
        self.comp_iterator.curr_loc()
    }

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

    fn curr_op(&self) -> Option<&Operator> {
        if self.comp_iterator.end() {
            None
        } else {
            if let Location::Component {
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
    }

    fn get_injected_val(&self, idx: usize) -> &Operator {
        if let Location::Component {
            mod_idx,
            func_idx,
            instr_idx,
        } = self.comp_iterator.curr_loc()
        {
            &self.comp.modules[mod_idx].code_sections[func_idx].instructions[instr_idx]
                .1
                .get_instr(idx)
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }
    }
}
