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

pub struct ComponentIterator<'a, 'b> {
    component: &'a mut Component<'b>,
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
        self.curr_instr + 1 < self.num_instr
    }

    fn next<'a>(&'a mut self, body: &'a Body) -> Option<&Operator> {
        if !self.has_next() {
            None
        } else {
            self.curr_instr += 1;
            let instr = &body.instructions[self.curr_instr];
            Some(&instr.0)
        }
    }

    fn curr_op<'a>(&'a self, body: &'a Body) -> Option<&Operator> {
        if self.curr_instr < self.num_instr {
            let instr = &body.instructions[self.curr_instr];
            Some(&instr.0)
        } else {
            None
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
        } else if self.next_function(module) {
            self.func_iterator
                .curr_op(&module.code_sections[self.curr_func])
        } else {
            None
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

impl<'a, 'b> ComponentIterator<'a, 'b> {
    pub fn new(comp: &'a mut Component<'b>) -> Self {
        // initializes to the first module
        let num_funcs = comp.modules[0].num_functions;
        let num_instrs = comp.modules[0].code_sections[0].num_instructions;
        ComponentIterator {
            component: comp,
            curr_mod: 0,
            mod_iterator: ModuleIterator::new(num_funcs, num_instrs),
        }
    }

    // // creats component iterator for a standalone module
    // // TODO: should we extend the module iterator instead of casting a module to a component?
    // TODO: can't create a new Component here, how should we iterate over just a module
    // without casting it to a Component
    // pub fn from_module(module: &'a mut Module<'a>) -> Self {
    //     // create a fresh component
    //     let mut comp = Component {
    //          modules : vec![module.clone()],
    //          core_types : vec![],
    //          component_types : vec![],
    //          imports : vec![],
    //          exports : vec![],
    //          instances : vec![],
    //          canons : vec![],
    //          alias : vec![],
    //          component_instance : vec![],
    //          custom_sections : vec![],
    //         num_modules : 1,
    //     };
    //     let num_funcs = comp.modules[0].num_functions;
    //     let num_instrs = comp.modules[0].code_sections[0].num_instructions;
    //     ComponentIterator {
    //         component: &mut comp,
    //         curr_mod: 0,
    //         mod_iterator: ModuleIterator::new(module.num_functions, module.code_sections[0].num_instructions),
    //     }
    // }

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

    // TODO: How much we want to delegate to module/func iterators?
    pub fn curr_mod_idx(&self) -> usize {
        self.curr_mod
    }

    pub fn curr_func_idx(&self) -> usize {
        self.mod_iterator.curr_func
    }

    pub fn curr_instr_idx(&self) -> usize {
        self.mod_iterator.func_iterator.curr_instr
    }

    pub fn curr_op(&self) -> Option<&Operator> {
        self.component.modules[self.curr_mod].code_sections[self.mod_iterator.curr_func]
            .instructions
            .get(self.mod_iterator.func_iterator.curr_instr)
            .map(|instr| &instr.0)
    }

    pub fn curr_instr_type(&self) -> &InstrumentType {
        &self.component.modules[self.curr_mod].code_sections[self.mod_iterator.curr_func]
            .instructions[self.mod_iterator.func_iterator.curr_instr]
            .1
    }

    // TODO: should we implement the std::iter::Iterator trait instead?
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&Operator> {
        if self.mod_iterator.has_next() {
            self.mod_iterator
                .next(&self.component.modules[self.curr_mod])
        } else if self.next_module() {
            self.curr_op()
        } else {
            None
        }
    }

    pub fn get_instrument_type(&self) -> &InstrumentType {
        self.mod_iterator
            .get_instrument_type(&self.component.modules[self.curr_mod])
    }

    pub fn before(&mut self) -> &mut Self {
        self.mod_iterator
            .before(&mut self.component.modules[self.curr_mod]);
        self
    }

    pub fn after(&mut self) -> &mut Self {
        self.mod_iterator
            .after(&mut self.component.modules[self.curr_mod]);
        self
    }

    pub fn alternate(&mut self) -> &mut Self {
        self.mod_iterator
            .alternate(&mut self.component.modules[self.curr_mod]);
        self
    }

    pub fn inject(&mut self, instr: Operator<'b>) {
        self.mod_iterator
            .inject(&mut self.component.modules[self.curr_mod], instr);
    }

    // Control Flow
    pub fn call(&mut self, idx: u32) -> &mut Self {
        self.inject(Operator::Call {
            function_index: idx,
        });
        self
    }

    pub fn return_stmt(&mut self) -> &mut Self {
        self.inject(Operator::Return);
        self
    }

    pub fn nop(&mut self) -> &mut Self {
        self.inject(Operator::Nop);
        self
    }

    pub fn unreachable(&mut self) -> &mut Self {
        self.inject(Operator::Unreachable);
        self
    }

    pub fn if_stmt(&mut self, block_type: wasmparser::ValType) -> &mut Self {
        self.inject(Operator::If {
            blockty: wasmparser::BlockType::Type(block_type),
        });
        self
    }

    pub fn else_stmt(&mut self) -> &mut Self {
        self.inject(Operator::Else);
        self
    }

    pub fn end(&mut self) -> &mut Self {
        self.inject(Operator::End);
        self
    }

    pub fn block(&mut self, block_type: wasmparser::BlockType) -> &mut Self {
        self.inject(Operator::Block {
            blockty: block_type,
        });
        self
    }

    pub fn loop_stmt(&mut self, block_type: wasmparser::BlockType) -> &mut Self {
        self.inject(Operator::Loop {
            blockty: block_type,
        });
        self
    }

    pub fn br(&mut self, relative_depth: u32) -> &mut Self {
        self.inject(Operator::Br { relative_depth });
        self
    }

    pub fn br_if(&mut self, relative_depth: u32) -> &mut Self {
        self.inject(Operator::BrIf { relative_depth });
        self
    }

    // Numerics
    pub fn local_get(&mut self, idx: u32) -> &mut Self {
        self.inject(Operator::LocalGet { local_index: idx });
        self
    }

    pub fn local_set(&mut self, idx: u32) -> &mut Self {
        self.inject(Operator::LocalSet { local_index: idx });
        self
    }

    // Integers
    pub fn i32(&mut self, value: i32) -> &mut Self {
        self.inject(Operator::I32Const { value });
        self
    }

    pub fn i32_add(&mut self) -> &mut Self {
        self.inject(Operator::I32Add);
        self
    }

    pub fn i32_sub(&mut self) -> &mut Self {
        self.inject(Operator::I32Sub);
        self
    }

    pub fn i32_mul(&mut self) -> &mut Self {
        self.inject(Operator::I32Mul);
        self
    }

    pub fn i32_div_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32DivS);
        self
    }

    pub fn i32_div_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32DivU);
        self
    }

    pub fn i32_rem_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32RemU);
        self
    }

    pub fn i32_rem_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32RemS);
        self
    }

    pub fn i32_and(&mut self) -> &mut Self {
        self.inject(Operator::I32And);
        self
    }

    pub fn i32_or(&mut self) -> &mut Self {
        self.inject(Operator::I32Or);
        self
    }

    pub fn i32_xor(&mut self) -> &mut Self {
        self.inject(Operator::I32Xor);
        self
    }

    pub fn i32_shl(&mut self) -> &mut Self {
        self.inject(Operator::I32Shl);
        self
    }

    pub fn i32_shr_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32ShrS);
        self
    }

    pub fn i32_shr_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32ShrU);
        self
    }

    pub fn i32_rotl(&mut self) -> &mut Self {
        self.inject(Operator::I32Rotl);
        self
    }

    pub fn i32_rotr(&mut self) -> &mut Self {
        self.inject(Operator::I32Rotr);
        self
    }

    pub fn i32_eq(&mut self) -> &mut Self {
        self.inject(Operator::I32Eq);
        self
    }

    pub fn i32_eqz(&mut self) -> &mut Self {
        self.inject(Operator::I32Eqz);
        self
    }

    pub fn i32_ne(&mut self) -> &mut Self {
        self.inject(Operator::I32Ne);
        self
    }

    pub fn i32_lt_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32LtU);
        self
    }

    pub fn i32_lt_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32LtS);
        self
    }

    pub fn i32_gt_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32GtU);
        self
    }

    pub fn i32_gt_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32GtS);
        self
    }

    pub fn i32_lte_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32LeU);
        self
    }

    pub fn i32_lte_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32LeS);
        self
    }

    pub fn i32_gte_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32GeU);
        self
    }

    pub fn i32_gte_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32GeS);
        self
    }

    pub fn i64(&mut self, value: i64) -> &mut Self {
        self.inject(Operator::I64Const { value });
        self
    }

    pub fn i64_add(&mut self) -> &mut Self {
        self.inject(Operator::I64Add);
        self
    }

    pub fn i64_sub(&mut self) -> &mut Self {
        self.inject(Operator::I64Sub);
        self
    }

    pub fn i64_mul(&mut self) -> &mut Self {
        self.inject(Operator::I64Mul);
        self
    }

    pub fn i64_div_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64DivS);
        self
    }

    pub fn i64_div_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64DivU);
        self
    }

    pub fn i64_rem_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64RemU);
        self
    }

    pub fn i64_rem_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64RemS);
        self
    }

    pub fn i64_and(&mut self) -> &mut Self {
        self.inject(Operator::I64And);
        self
    }

    pub fn i64_or(&mut self) -> &mut Self {
        self.inject(Operator::I64Or);
        self
    }

    pub fn i64_xor(&mut self) -> &mut Self {
        self.inject(Operator::I64Xor);
        self
    }

    pub fn i64_shl(&mut self) -> &mut Self {
        self.inject(Operator::I64Shl);
        self
    }

    pub fn i64_shr_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64ShrS);
        self
    }

    pub fn i64_shr_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64ShrU);
        self
    }

    pub fn i64_rotl(&mut self) -> &mut Self {
        self.inject(Operator::I64Rotl);
        self
    }

    pub fn i64_rotr(&mut self) -> &mut Self {
        self.inject(Operator::I64Rotr);
        self
    }

    pub fn i64_eq(&mut self) -> &mut Self {
        self.inject(Operator::I64Eq);
        self
    }

    pub fn i64_eqz(&mut self) -> &mut Self {
        self.inject(Operator::I64Eqz);
        self
    }

    pub fn i64_ne(&mut self) -> &mut Self {
        self.inject(Operator::I64Ne);
        self
    }

    pub fn i64_lt_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64LtU);
        self
    }

    pub fn i64_lt_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64LtS);
        self
    }

    pub fn i64_gt_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64GtU);
        self
    }

    pub fn i64_gt_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64GtS);
        self
    }

    pub fn i64_lte_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64LeU);
        self
    }

    pub fn i64_lte_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64LeS);
        self
    }

    pub fn i64_gte_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64GeU);
        self
    }

    pub fn i64_gte_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64GeS);
        self
    }

    // Floating point
    pub fn f32(&mut self, val: f32) -> &mut Self {
        self.inject(Operator::F32Const {
            value: wasmparser::Ieee32::from(val),
        });
        self
    }

    pub fn f64(&mut self, val: f64) -> &mut Self {
        self.inject(Operator::F64Const {
            value: wasmparser::Ieee64::from(val),
        });
        self
    }

    // Memory Instructions
    pub fn memory_init(&mut self, data_index: u32, mem: u32) -> &mut Self {
        self.inject(Operator::MemoryInit { data_index, mem });
        self
    }

    pub fn memory_size(&mut self, mem: u32) -> &mut Self {
        self.inject(Operator::MemorySize { mem });
        self
    }

    pub fn memory_grow(&mut self, mem: u32) -> &mut Self {
        self.inject(Operator::MemoryGrow { mem });
        self
    }

    pub fn memory_fill(&mut self, mem: u32) -> &mut Self {
        self.inject(Operator::MemoryFill { mem });
        self
    }

    pub fn memory_copy(&mut self, dst_mem: u32, src_mem: u32) -> &mut Self {
        self.inject(Operator::MemoryCopy { dst_mem, src_mem });
        self
    }

    pub fn memory_discard(&mut self, mem: u32) -> &mut Self {
        self.inject(Operator::MemoryDiscard { mem });
        self
    }

    pub fn data_drop(&mut self, data_index: u32) -> &mut Self {
        self.inject(Operator::DataDrop { data_index });
        self
    }

    // Parametric Instructions
    pub fn drop(&mut self) -> &mut Self {
        self.inject(Operator::Drop);
        self
    }

    pub fn get_injected_val(&mut self, idx: usize) -> &Operator {
        self.mod_iterator
            .get_injected_val(idx, &self.component.modules[self.curr_mod])
    }
}
