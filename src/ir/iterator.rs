use crate::ir::component::Component;
use crate::ir::module::Module;
use crate::ir::types::InstrumentType::{InstrumentAfter, InstrumentAlternate, InstrumentBefore};
use crate::ir::types::{Body, InstrumentType};
use wasmparser::Operator;

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

    fn reset(&mut self, num_instr: usize) {
        self.curr_instr = 0;
        self.num_instr = num_instr;
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

    fn get_instrument_type<'a>(&'a self, body: &'a Body) -> &InstrumentType {
        &body.instructions[self.curr_instr].1
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

    pub fn inject<'a>(&mut self, body: &mut Body<'a>, instr: Operator<'a>) {
        body.instructions[self.curr_instr].1.add_instr(instr);
    }

    pub fn get_injected_val<'a>(&'a self, idx: usize, body: &'a Body) -> &Operator {
        body.instructions[self.curr_instr].1.get_instr(idx)
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

    fn reset(&mut self, num_funcs: usize, num_instr: usize) {
        self.curr_func = 0;
        self.num_funcs = num_funcs;
        self.func_iterator.reset(num_instr);
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

    fn get_instrument_type<'a>(&'a self, module: &'a Module) -> &InstrumentType {
        self.func_iterator
            .get_instrument_type(&module.code_sections[self.curr_func])
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

    pub fn inject<'a>(&mut self, module: &mut Module<'a>, instr: Operator<'a>) {
        self.func_iterator
            .inject(&mut module.code_sections[self.curr_func], instr);
    }

    pub fn get_injected_val<'a>(&'a self, idx: usize, module: &'a Module) -> &Operator {
        self.func_iterator
            .get_injected_val(idx, &module.code_sections[self.curr_func])
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

    pub fn reset(&mut self) {
        self.curr_mod = 0;
        self.mod_iterator.reset(
            self.component.modules[self.curr_mod].num_functions,
            self.component.modules[self.curr_mod].code_sections[0].num_instructions,
        );
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

    pub fn get_instrument_type(&self) -> &InstrumentType {
        self.mod_iterator
            .get_instrument_type(&self.component.modules[self.curr_mod])
    }

    pub fn before(&mut self) {
        self.mod_iterator
            .before(&mut self.component.modules[self.curr_mod])
    }

    pub fn after(&mut self) {
        self.mod_iterator
            .after(&mut self.component.modules[self.curr_mod])
    }

    pub fn alternate(&mut self) {
        self.mod_iterator
            .alternate(&mut self.component.modules[self.curr_mod])
    }

    pub fn inject(&mut self, instr: Operator<'a>) {
        self.mod_iterator
            .inject(&mut self.component.modules[self.curr_mod], instr);
    }

    pub fn i32(&mut self, value: i32) {
        self.inject(Operator::I32Const { value });
    }

    pub fn get_injected_val(&mut self, idx: usize) -> &Operator {
        self.mod_iterator
            .get_injected_val(idx, &self.component.modules[self.curr_mod])
    }
}
