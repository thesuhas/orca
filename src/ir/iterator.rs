use crate::ir::component::Component;
use crate::ir::module::Module;
use crate::ir::types::Body;
use crate::ir::types::InstrumentType::{InstrumentAfter, InstrumentAlternate, InstrumentBefore};
use wasmparser::Operator;

// pub trait Iterator {
//     fn next(&mut self) -> Option<&Operator>;
//
//     // These methods enable instrumentation
//     fn before(&mut self);
//
//     fn after(&mut self);
//
//     fn alternate(&mut self);
// }

pub struct FuncIterator {
    curr_instr: usize,
    num_instr: usize,
}

pub struct ModuleIterator {
    curr_func: usize,
    num_funcs: usize,
    func_iterator: FuncIterator,
}

pub struct ComponentIterator<'a> {
    component: &'a mut Component<'a>,
    curr_mod: usize,
    mod_iterator: ModuleIterator,
}

impl FuncIterator {
    pub fn new(num_instr: usize) -> Self {
        FuncIterator {
            curr_instr: 0,
            num_instr,
        }
    }

    fn has_next(&self) -> bool {
        self.curr_instr < self.num_instr
    }

    fn next<'a>(&'a mut self, body: &'a Body) -> Option<&Operator> {
        if !self.has_next() {
            None
        } else {
            let instr = &body.instructions[self.curr_instr];
            println!(
                "Instruction: {:?} Instrumented Type: {:?}",
                instr.0, instr.1
            );
            self.curr_instr += 1;
            Some(&instr.0)
        }
    }

    fn before(&self, body: &mut Body) {
        body.instructions[self.curr_instr].1 = InstrumentBefore(vec![])
    }

    fn after(&self, body: &mut Body) {
        body.instructions[self.curr_instr].1 = InstrumentAfter(vec![])
    }

    fn alternate(&self, body: &mut Body) {
        body.instructions[self.curr_instr].1 = InstrumentAlternate(vec![])
    }
}

impl ModuleIterator {
    pub fn new(num_funcs: usize, num_instr: usize) -> Self {
        ModuleIterator {
            curr_func: 0,
            num_funcs,
            func_iterator: FuncIterator::new(num_instr),
        }
    }

    pub fn has_next_function(&self) -> bool {
        self.curr_func + 1 < self.num_funcs
    }

    fn next_function(&mut self, module: &Module) -> bool {
        self.curr_func += 1;
        if self.curr_func < self.num_funcs {
            self.func_iterator =
                FuncIterator::new(module.code_sections[self.curr_func].num_instructions);
            true
        } else {
            false
        }
    }

    fn has_next(&self) -> bool {
        self.func_iterator.has_next() || self.has_next_function()
    }

    fn next<'a>(&'a mut self, module: &'a Module) -> Option<&Operator> {
        if self.func_iterator.has_next() {
            self.func_iterator
                .next(&module.code_sections[self.curr_func])
        } else {
            if self.next_function(module) {
                self.func_iterator
                    .next(&module.code_sections[self.curr_func])
            } else {
                None
            }
        }
    }

    fn before(&mut self, module: &mut Module) {
        self.func_iterator
            .before(&mut module.code_sections[self.curr_func])
    }

    fn after(&mut self, module: &mut Module) {
        self.func_iterator
            .after(&mut module.code_sections[self.curr_func])
    }

    fn alternate(&mut self, module: &mut Module) {
        self.func_iterator
            .alternate(&mut module.code_sections[self.curr_func])
    }
}

impl<'a> ComponentIterator<'a> {
    pub fn new(comp: &'a mut Component<'a>) -> Self {
        let num_modules = comp.modules[0].num_functions;
        let num_functions = comp.modules[0].code_sections[0].num_instructions;
        ComponentIterator {
            component: comp,
            curr_mod: 0,
            mod_iterator: ModuleIterator::new(num_modules, num_functions),
        }
    }

    fn next_module(&mut self) -> bool {
        self.curr_mod += 1;
        if self.curr_mod < self.component.num_modules {
            // If we're defining a new module, we have to reset function
            self.mod_iterator = ModuleIterator::new(
                self.component.modules[self.curr_mod].num_functions,
                self.component.modules[self.curr_mod].code_sections[0].num_instructions,
            );
            true
        } else {
            false
        }
    }

    pub fn next(&mut self) -> Option<&Operator> {
        if self.mod_iterator.has_next() {
            self.mod_iterator
                .next(&self.component.modules[self.curr_mod])
        } else {
            if self.next_module() {
                self.mod_iterator
                    .next(&self.component.modules[self.curr_mod])
            } else {
                None
            }
        }
    }

    fn before(&mut self) {
        self.mod_iterator
            .before(&mut self.component.modules[self.curr_mod])
    }

    fn after(&mut self) {
        self.mod_iterator
            .after(&mut self.component.modules[self.curr_mod])
    }

    fn alternate(&mut self) {
        self.mod_iterator
            .alternate(&mut self.component.modules[self.curr_mod])
    }
}
