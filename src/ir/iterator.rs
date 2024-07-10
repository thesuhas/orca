use crate::ir::component::Component;
use crate::ir::module::Module;
use crate::ir::types::Body;
use wasmparser::Operator;

pub struct FuncIterator<'a> {
    curr_instr: usize,
    function: &'a Body<'a>,
}

pub struct ModuleIterator<'a> {
    curr_func: usize,
    module: &'a Module<'a>,
    func_iterator: FuncIterator<'a>,
}

pub struct ComponentIterator<'a> {
    component: &'a Component<'a>,
    curr_mod: usize,
    mod_iterator: ModuleIterator<'a>,
}

impl<'a> FuncIterator<'a> {
    pub fn new(func: &'a Body<'a>) -> Self {
        FuncIterator {
            curr_instr: 0,
            function: func,
        }
    }

    pub fn has_next(&self) -> bool {
        self.curr_instr < self.function.num_instructions
    }

    pub fn next(&mut self) -> Option<&Operator> {
        if !self.has_next() {
            None
        } else {
            let instr = &self.function.instructions[self.curr_instr];
            println!(
                "Instruction: {:?} Instrumented Type: {:?}",
                instr.0, instr.1
            );
            self.curr_instr += 1;
            Some(&instr.0)
        }
    }
}

impl<'a> ModuleIterator<'a> {
    pub fn new(module: &'a Module<'a>) -> Self {
        ModuleIterator {
            curr_func: 0,
            module,
            func_iterator: FuncIterator::new(&module.code_sections[0]),
        }
    }

    pub fn has_next_function(&self) -> bool {
        self.curr_func + 1 < self.module.num_functions
    }

    fn next_function(&mut self) -> bool {
        self.curr_func += 1;
        if self.curr_func < self.module.num_functions {
            self.func_iterator = FuncIterator::new(&self.module.code_sections[self.curr_func]);
            true
        } else {
            false
        }
    }

    pub fn has_next(&self) -> bool {
        self.func_iterator.has_next() || self.has_next_function()
    }

    pub fn next(&mut self) -> Option<&Operator> {
        if self.func_iterator.has_next() {
            self.func_iterator.next()
        } else {
            if self.next_function() {
                self.func_iterator.next()
            } else {
                None
            }
        }
    }
}


impl<'a> ComponentIterator<'a> {
    pub fn new(comp: &'a Component<'a>) -> Self {
        ComponentIterator {
            component: comp,
            curr_mod: 0,
            mod_iterator: ModuleIterator::new(&comp.modules[0]),
        }
    }

    fn next_module(&mut self) -> bool {
        self.curr_mod += 1;
        if self.curr_mod < self.component.num_modules {
            self.mod_iterator = ModuleIterator::new(&self.component.modules[self.curr_mod]);
            true
        } else {
            false
        }
    }

    pub fn next(&mut self) -> Option<&Operator> {
        if self.mod_iterator.has_next() {
            self.mod_iterator.next()
        } else {
            if self.next_module() {
                self.mod_iterator.next()
            } else {
                None
            }
        }
    }
}

