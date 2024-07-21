//! Intermediate Representation of a fucntion

use crate::ir::module::Module;
use crate::ir::types::Body;
use crate::ir::types::DataType;
use crate::ir::types::{Instrument, InstrumentType, InstrumentationMode};
use crate::opcode::Opcode;
use crate::ModuleBuilder;
use wasmparser::Operator;

// TODO: probably need better reasoning with lifetime here
/// Build a function from scratch
/// See example at `fac_orca/src/main.rs`
pub struct FunctionBuilder<'a> {
    // pub(crate) id: u32, // function index
    pub(crate) params: Vec<DataType>,
    pub(crate) results: Vec<DataType>,
    #[allow(dead_code)]
    // TODO: how to add function name?
    // https://docs.rs/walrus/latest/walrus/struct.FunctionBuilder.html#method.name
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
    pub fn finish(mut self, module: &mut Module<'a>) -> u32 {
        // add End as last instruction
        self.end();

        let ty = module.add_type(&self.params, &self.results);

        // the function index is should also take account for imports
        let id = module.functions.len() as u32 + module.imports.len() as u32;
        module.functions.push(ty);
        module.code_sections.push(self.body);
        module.num_functions += 1;

        assert_eq!(module.functions.len(), module.code_sections.len());
        assert_eq!(module.functions.len(), module.num_functions);
        id
    }

    /// add a local and return local index
    /// (note that local indices start after)
    pub fn add_local(&mut self, ty: DataType) -> u32 {
        let index = self.params.len() as u32 + self.body.num_locals as u32;
        self.body.num_locals += 1;
        // if ValType exists in body.locals, increment count
        // we assume this at most get applied at one place
        let mut found_ty = false;
        for x in self.body.locals.iter_mut() {
            if x.1 == ty {
                x.0 += 1;
                found_ty = true;
                break;
            }
        }
        if !found_ty {
            self.body.locals.push((1, ty));
        }

        index
    }
}

impl<'a> Opcode<'a> for FunctionBuilder<'a> {
    /// Inject an operator at the end of the function
    // here the location of the injection is always at the end of the function
    fn inject(&mut self, instr: Operator<'a>) {
        self.body.push_instr(instr)
    }
}

impl ModuleBuilder for FunctionBuilder<'_> {
    fn add_local(&mut self, ty: DataType) -> u32 {
        FunctionBuilder::add_local(self, ty)
    }
}

/// Modify a function
/// Uses same injection logic as Iterator, which is different from
/// FunctionBuilder since FunctionModifier does side effect to operators at encoding
/// (it only modifies the Instrument type)
pub struct FunctionModifier<'a, 'b> {
    pub(crate) body: &'a mut Body<'b>,
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
        self.set_instrument_type(idx, InstrumentationMode::Before);
        self
    }

    /// adding instructions after the specified instruction
    pub fn after_at(&mut self, idx: usize) -> &mut Self {
        self.set_instrument_type(idx, InstrumentationMode::After);
        self
    }

    /// adding instructions alternate to the specified instruction
    pub fn alternate_at(&mut self, idx: usize) -> &mut Self {
        self.set_instrument_type(idx, InstrumentationMode::Alternate);
        self
    }

    fn set_instrument_type(&mut self, idx: usize, mode: InstrumentationMode) {
        {
            self.instr_idx = Some(idx);
            if self.body.instructions[idx].1.get_curr() == InstrumentType::NotInstrumented {
                self.body.instructions[idx].1 = Instrument::Instrumented {
                    before: vec![],
                    after: vec![],
                    alternate: vec![],
                    current: mode,
                }
            } else {
                self.body.instructions[idx].1.set_curr(mode);
            }
        }
    }
}

impl<'a, 'b> Opcode<'b> for FunctionModifier<'a, 'b> {
    // TODO: refactor the inject the function to return a Result rather than panicking?
    fn inject(&mut self, instr: Operator<'b>) {
        if let Some(idx) = self.instr_idx {
            self.body.instructions[idx].1.add_instr(instr);
        } else {
            panic!("Instruction index not set");
        }
    }
}