use crate::ir::iterator::function_iterator::FuncIterator;
use crate::ir::module::Module;
use crate::ir::types::InstrumentType;
use wasmparser::Operator;

pub struct ModuleIterator {
    pub(crate) curr_func: usize,
    num_funcs: usize,
    pub(crate) func_iterator: FuncIterator,
}

impl ModuleIterator {
    pub fn new(num_funcs: usize, num_instr: usize) -> Self {
        ModuleIterator {
            curr_func: 0,
            num_funcs,
            func_iterator: FuncIterator::new(num_instr),
        }
    }

    pub(crate) fn reset(&mut self, num_funcs: usize, num_instr: usize) {
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

    pub(crate) fn get_instrument_type<'a>(&'a self, module: &'a Module) -> &InstrumentType {
        self.func_iterator
            .get_instrument_type(&module.code_sections[self.curr_func])
    }

    pub(crate) fn has_next(&self) -> bool {
        self.func_iterator.has_next() || self.has_next_function()
    }

    pub(crate) fn next<'a>(&'a mut self, module: &'a Module) -> Option<&Operator> {
        if self.func_iterator.has_next() {
            self.func_iterator
                .next(&module.code_sections[self.curr_func])
        } else if self.next_function(module) {
            self.func_iterator
                .curr_op(&module.code_sections[self.curr_func])
        } else {
            None
        }
    }

    pub(crate) fn before(&mut self, module: &mut Module) {
        self.func_iterator
            .before(&mut module.code_sections[self.curr_func])
    }

    pub(crate) fn after(&mut self, module: &mut Module) {
        self.func_iterator
            .after(&mut module.code_sections[self.curr_func])
    }

    pub(crate) fn alternate(&mut self, module: &mut Module) {
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
