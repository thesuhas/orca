//! Function Builder

use crate::ir::id::{FunctionID, LocalID, ModuleID, TypeID};
use crate::ir::module::module_functions::FuncKind::Local;
use crate::ir::module::module_functions::{add_local, Function, LocalFunction};
use crate::ir::module::Module;
use crate::ir::types::DataType;
use crate::ir::types::InstrumentationMode;
use crate::ir::types::{Body, FuncInstrFlag, FuncInstrMode};
use crate::module_builder::AddLocal;
use crate::opcode::{Inject, Instrumenter, MacroOpcode, Opcode};
use crate::{Component, Location};
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
            body: Body::default(),
        }
    }

    /// Finish building a function (have side effect on module IR),
    /// return function index
    pub fn finish_module(mut self, num_args: usize, module: &mut Module<'a>) -> FunctionID {
        let mut args = vec![];
        for arg in 0..num_args {
            args.push(arg as LocalID);
        }
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

    fn inject_at(&mut self, _idx: usize, _mode: InstrumentationMode, _instr: Operator<'a>) {
        // self.set_instrument_mode_at(mode, Location::Module {
        //     func_idx: 0, // not used
        //     instr_idx: idx
        // });
        // self.body.instructions[idx].1.add_instr(instr);
        unreachable!()
    }
}
impl<'a> Opcode<'a> for FunctionBuilder<'a> {}
impl<'a> MacroOpcode<'a> for FunctionBuilder<'a> {}

impl AddLocal for FunctionBuilder<'_> {
    /// add a local and return local index
    /// (note that local indices start after)
    fn add_local(&mut self, ty: DataType) -> LocalID {
        add_local(
            ty,
            self.params.len(),
            &mut self.body.num_locals,
            &mut self.body.locals,
        )
    }
}

/// Modify a function
/// Uses same injection logic as Iterator, which is different from
/// FunctionBuilder since FunctionModifier does side effect to operators at encoding
/// (it only modifies the Instrument type)
pub struct FunctionModifier<'a, 'b> {
    pub instr_flag: FuncInstrFlag<'a>,
    pub body: &'a mut Body<'b>,
    pub args: &'a mut Vec<LocalID>,
    pub(crate) instr_idx: Option<usize>,
}

impl<'a, 'b> FunctionModifier<'a, 'b> {
    // by default, the instr_idx the last instruction (always Operator::End indicating end of the function)
    // and the Instrument type is set to before
    pub fn init(body: &'a mut Body<'b>, args: &'a mut Vec<LocalID>) -> Self {
        let instr_idx = body.instructions.len() - 1;
        let mut func_modifier = FunctionModifier {
            instr_flag: FuncInstrFlag::default(),
            body,
            args,
            instr_idx: None,
        };
        func_modifier.before_at(Location::Module {
            func_idx: 0, // not used
            instr_idx,
        });
        func_modifier
    }

    /// add a local and return local index
    pub fn add_local(&mut self, ty: DataType) -> LocalID {
        add_local(
            ty,
            self.args.len(),
            &mut self.body.num_locals,
            &mut self.body.locals,
        )
    }
}

impl<'a, 'b> Inject<'b> for FunctionModifier<'a, 'b> {
    // TODO: refactor the inject the function to return a Result rather than panicking?
    fn inject(&mut self, instr: Operator<'b>) {
        if self.instr_flag.current_mode.is_some() {
            // inject at the function level
            self.instr_flag.add_instr(instr);
        } else {
            // inject at instruction level
            if let Some(idx) = self.instr_idx {
                let is_special = self.body.instructions[idx].instr_flag.add_instr(instr);
                // remember if we injected a special instrumentation (to be resolved before encoding)
                self.instr_flag.has_special_instr |= is_special;
            } else {
                panic!("Instruction index not set");
            }
        }
    }

    fn inject_at(&mut self, idx: usize, mode: InstrumentationMode, instr: Operator<'b>) {
        let loc = Location::Module {
            func_idx: 0, // not used
            instr_idx: idx,
        };
        self.set_instrument_mode_at(mode, loc);
        self.add_instr_at(loc, instr);
    }
}
impl<'a, 'b> Opcode<'b> for FunctionModifier<'a, 'b> {}
impl<'a, 'b> MacroOpcode<'b> for FunctionModifier<'a, 'b> {}

impl<'a, 'b> Instrumenter<'b> for FunctionModifier<'a, 'b> {
    fn curr_instrument_mode(&self) -> &Option<InstrumentationMode> {
        if let Some(idx) = self.instr_idx {
            &self.body.instructions[idx].instr_flag.current_mode
        } else {
            panic!("Instruction index not set");
        }
    }

    fn set_instrument_mode_at(&mut self, mode: InstrumentationMode, loc: Location) {
        if let Location::Module { instr_idx, .. } = loc {
            self.instr_idx = Some(instr_idx);
            self.body.instructions[instr_idx].instr_flag.current_mode = Some(mode);
        } else {
            panic!("Should have gotten module location");
        }
    }

    fn curr_func_instrument_mode(&self) -> &Option<FuncInstrMode> {
        &self.instr_flag.current_mode
    }

    fn set_func_instrument_mode(&mut self, mode: FuncInstrMode) {
        self.instr_flag.current_mode = Some(mode);
    }

    fn add_instr_at(&mut self, loc: Location, instr: Operator<'b>) {
        if let Location::Module { instr_idx, .. } = loc {
            self.body.instructions[instr_idx]
                .instr_flag
                .add_instr(instr);
        } else {
            panic!("Should have gotten module location");
        }
    }

    fn empty_alternate_at(&mut self, loc: Location) -> &mut Self {
        if let Location::Module { instr_idx, .. } = loc {
            self.body.instructions[instr_idx].instr_flag.alternate = Some(vec![]);
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }

        self
    }

    fn empty_block_alt_at(&mut self, loc: Location) -> &mut Self {
        if let Location::Module { instr_idx, .. } = loc {
            self.body.instructions[instr_idx].instr_flag.block_alt = Some(vec![]);
            self.instr_flag.has_special_instr |= true;
        } else {
            panic!("Should have gotten Component Location and not Module Location!")
        }

        self
    }

    fn get_injected_val(&self, idx: usize) -> &Operator {
        self.body.instructions[idx].instr_flag.get_instr(idx)
    }
}
