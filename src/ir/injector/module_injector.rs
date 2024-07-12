use crate::ir::injector::injector_trait::Injector;
use crate::ir::iterator::module_iterator::ModuleIterator;
use crate::ir::module::Module;
use crate::ir::types::{InstrumentType, Location};
use std::collections::HashMap;
use wasmparser::Operator;

pub struct ModuleInjector<'a, 'b> {
    module: &'a mut Module<'b>,
    mod_iterator: ModuleIterator,
}

#[allow(dead_code)]
impl<'a, 'b> ModuleInjector<'a, 'b> {
    pub fn new(module: &'a mut Module<'b>) -> Self {
        // Creates Function -> Number of Instructions
        let mut metadata = HashMap::new();
        for (idx, func) in module.code_sections.iter().enumerate() {
            metadata.insert(idx, func.num_instructions);
        }
        let num_funcs = module.num_functions;
        ModuleInjector {
            module,
            mod_iterator: ModuleIterator::new(num_funcs, metadata),
        }
    }
}

// Note: Marked Trait as the same lifetime as component
impl<'a, 'b> Injector<'b> for ModuleInjector<'a, 'b> {
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

    fn reset(&mut self) {
        self.mod_iterator.reset();
    }

    fn next(&mut self) -> Option<&Operator> {
        match self.mod_iterator.next() {
            false => None,
            true => self.curr_op(),
        }
    }

    fn curr_loc(&self) -> Location {
        self.mod_iterator.curr_loc()
    }

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

    fn curr_op(&self) -> Option<&Operator> {
        if self.mod_iterator.end() {
            None
        } else {
            if let Location::Module {
                func_idx,
                instr_idx,
            } = self.mod_iterator.curr_loc()
            {
                Some(&self.module.code_sections[func_idx].instructions[instr_idx].0)
            } else {
                panic!("Should have gotten Module Location!")
            }
        }
    }

    fn get_injected_val(&self, idx: usize) -> &Operator {
        if let Location::Module {
            func_idx,
            instr_idx,
        } = self.mod_iterator.curr_loc()
        {
            &self.module.code_sections[func_idx].instructions[instr_idx]
                .1
                .get_instr(idx)
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }
    }
}
