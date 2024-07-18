//! Intermediate Representation of a fucntion

use crate::ir::module::Module;
use crate::ir::types::Body;
use crate::ir::types::DataType;
use crate::opcode::Opcode;
use wasmparser::Operator;

// TODO: probably need better reasoning with lifetime here
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
    /// Inject an operator at the current location
    fn inject(&mut self, instr: Operator<'a>) {
        self.body.add_instr(instr)
    }
}
