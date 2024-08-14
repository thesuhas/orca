//! Function Builder

use crate::ir::id::{FunctionID, LocalID, ModuleID, TypeID};
use crate::ir::module::module_functions::FuncKind::Local;
use crate::ir::module::module_functions::{Function, LocalFunction};
use crate::ir::module::Module;
use crate::ir::types::Body;
use crate::ir::types::DataType;
use crate::ir::types::InstrumentationMode;
use crate::opcode::{Inject, MacroOpcode, Opcode};
use crate::{Component, ModuleBuilder};
use wasmparser::Operator;

// TODO: probably need better reasoning with lifetime here
/// Build a function from scratch
/// See an example [here].
///
/// [here]: https://github.com/thesuhas/orca/blob/314af2df01203e7715aa728e7388cf39c564e9d7/fac_orca/src/main.rs#L16
pub struct FunctionBuilder<'a> {
    // pub(crate) id: u32, // function index
    pub(crate) params: Vec<DataType>,
    pub(crate) results: Vec<DataType>,
    #[allow(dead_code)]
    pub(crate) name: Option<String>,
    pub body: Body<'a>,
}

impl<'a> FunctionBuilder<'a> {
    pub fn new(params: &[DataType], results: &[DataType]) -> Self {
        Self {
            params: params.to_vec(),
            results: results.to_vec(),
            name: None,
            body: Body::new(),
        }
    }

    /// Finish building a function (have side effect on module IR),
    /// return function index
    pub fn finish_module(mut self, args: Vec<LocalID>, module: &mut Module<'a>) -> FunctionID {
        // add End as last instruction
        self.end();

        let ty = module.types.add(&self.params, &self.results);

        let id = module.functions.len();

        let func = Function::new(
            Local(LocalFunction::new(
                ty,
                id as FunctionID,
                self.body.clone(),
                args,
            )),
            self.name,
        );
        module.functions.push(func);
        // module.code_sections.push(self.body);
        module.num_functions += 1;

        // assert_eq!(module.functions.len(), module.code_sections.len());
        assert_eq!(
            module.functions.len(),
            module.num_functions + module.num_imported_functions
        );
        id as FunctionID
    }

    /// Finish building a function (have side effect on component IR),
    /// return function index
    pub fn finish_component(
        mut self,
        args: Vec<LocalID>,
        comp: &mut Component<'a>,
        mod_idx: ModuleID,
    ) -> FunctionID {
        // add End as last instruction
        self.end();

        let ty = comp.modules[0].types.add(&self.params, &self.results);

        let id = comp.modules[mod_idx as usize].functions.len();

        let func = Function::new(
            Local(LocalFunction::new(
                ty,
                id as FunctionID,
                self.body.clone(),
                args,
            )),
            self.name,
        );

        // the function index should also take account for imports
        comp.modules[mod_idx as usize].functions.push(func);
        // comp.modules[mod_idx as usize].code_sections.push(self.body);
        comp.modules[mod_idx as usize].num_functions += 1;

        // assert_eq!(
        //     comp.modules[mod_idx as usize].functions.len(),
        //     comp.modules[mod_idx as usize].code_sections.len()
        // );
        assert_eq!(
            comp.modules[mod_idx as usize].functions.len(),
            comp.modules[mod_idx as usize].num_functions
                + comp.modules[mod_idx as usize].num_imported_functions
        );
        id as FunctionID
    }

    /// add a local and return local index
    /// (note that local indices start after)
    pub fn add_local(&mut self, ty: DataType) -> LocalID {
        let index = self.params.len() + self.body.num_locals;
        let len = self.body.locals.len();
        self.body.num_locals += 1;
        if len > 0 {
            let last = len - 1;
            if self.body.locals[last].1 == ty {
                self.body.locals[last].0 += 1;
            } else {
                self.body.locals.push((1, ty));
            }
        } else {
            // If no locals, just append
            self.body.locals.push((1, ty));
        }
        index as LocalID
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name)
    }

    pub fn local_func(
        &self,
        args: Vec<LocalID>,
        function_id: FunctionID,
        ty: TypeID,
    ) -> LocalFunction<'a> {
        LocalFunction::new(ty, function_id, self.body.clone(), args)
    }
}

impl<'a> Inject<'a> for FunctionBuilder<'a> {
    /// Inject an operator at the end of the function
    // here the location of the injection is always at the end of the function
    fn inject(&mut self, instr: Operator<'a>) {
        self.body.push_instr(instr)
    }
}
impl<'a> Opcode<'a> for FunctionBuilder<'a> {}
impl<'a> MacroOpcode<'a> for FunctionBuilder<'a> {}

impl ModuleBuilder for FunctionBuilder<'_> {
    fn add_local(&mut self, ty: DataType) -> LocalID {
        self.add_local(ty)
    }
}

/// Modify a function
/// Uses same injection logic as Iterator, which is different from
/// FunctionBuilder since FunctionModifier does side effect to operators at encoding
/// (it only modifies the Instrument type)
pub struct FunctionModifier<'a, 'b> {
    pub body: &'a mut Body<'b>,
    pub(crate) instr_idx: Option<usize>,
}

impl<'a, 'b> FunctionModifier<'a, 'b> {
    // by default, the instr_idx the last instruction (always Operator::End indicating end of the function)
    // and the Instrument type is set to before
    pub fn init(body: &'a mut Body<'b>) -> Self {
        let instr_idx = body.instructions.len() - 1;
        let mut func_modifier = FunctionModifier {
            body,
            instr_idx: None,
        };
        func_modifier.before_at(instr_idx);
        func_modifier
    }

    /// adding instructions before the specified instruction
    pub fn before_at(&mut self, idx: usize) -> &mut Self {
        self.set_instrument_mode(idx, InstrumentationMode::Before);
        self
    }

    /// adding instructions after the specified instruction
    pub fn after_at(&mut self, idx: usize) -> &mut Self {
        self.set_instrument_mode(idx, InstrumentationMode::After);
        self
    }

    /// adding instructions alternate to the specified instruction
    pub fn alternate_at(&mut self, idx: usize) -> &mut Self {
        self.set_instrument_mode(idx, InstrumentationMode::Alternate);
        self
    }

    pub fn inject_at(
        &mut self,
        idx: usize,
        mode: InstrumentationMode,
        instr: Operator<'b>,
    ) -> &mut Self {
        self.set_instrument_mode(idx, mode);
        self.body.instructions[idx].1.add_instr(instr);
        self
    }

    fn set_instrument_mode(&mut self, idx: usize, mode: InstrumentationMode) {
        self.instr_idx = Some(idx);
        self.body.instructions[idx].1.current_mode = Some(mode);
    }
}

impl<'a, 'b> Inject<'b> for FunctionModifier<'a, 'b> {
    // TODO: refactor the inject the function to return a Result rather than panicking?
    fn inject(&mut self, instr: Operator<'b>) {
        if let Some(idx) = self.instr_idx {
            self.body.instructions[idx].1.add_instr(instr);
        } else {
            panic!("Instruction index not set");
        }
    }
}
impl<'a, 'b> Opcode<'b> for FunctionModifier<'a, 'b> {}
impl<'a, 'b> MacroOpcode<'b> for FunctionModifier<'a, 'b> {}
