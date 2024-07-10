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

impl FuncIterator {
    pub fn new(func: &Body) -> Self {
        FuncIterator {
            curr_instr: 0,
            function: func,
        }
    }

    pub fn next(&mut self) -> Option<&Operator> {
        if self.curr_instr >= self.function.num_instructions {
            None
        } else {
            let instr = &self.function.instructions[self.curr_instr];
            self.curr_instr += 1;
            Some(&instr.0)
        }
    }
}

impl ModuleIterator {
    pub fn new(module: &Module) -> Self {
        ModuleIterator {
            curr_func: 0,
            module,
            func_iterator: FuncIterator::new(&module.code_sections[0]),
        }
    }

    pub fn next(&mut self) -> Option<&Operator> {
        if self.curr_func >= self.module.num_functions {
            None
        } else {
            let res = self.func_iterator.next();
            self.curr_func += 1;
            res
        }
    }
}

impl ComponentIterator {
    pub fn new(comp: &Component) -> Self {
        ComponentIterator {
            component: comp,
            curr_mod: 0,
            mod_iterator: ModuleIterator::new(&comp.modules[0]),
        }
    }

    pub fn next(&mut self) -> Option<&Operator> {
        if self.curr_mod >= self.component.num_modules {
            None
        } else {
            let res = self.mod_iterator.next();
            self.curr_mod += 1;
            res
        }
    }
}
