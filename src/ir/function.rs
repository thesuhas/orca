//! Intermediate Representation of a fucntion

use crate::ir::module::Module;
use crate::ir::types::Body;
use wasmparser::Operator;
use wasmparser::ValType;

// TODO: probably need better reasoning with lifetime here
pub struct FunctionBuilder<'a> {
    // pub(crate) id: u32, // function index
    pub(crate) params: Vec<ValType>,
    pub(crate) results: Vec<ValType>,
    pub(crate) num_locals: u32,
    #[allow(dead_code)]
    // TODO: how to add function name?
    // https://docs.rs/walrus/latest/walrus/struct.FunctionBuilder.html#method.name
    pub(crate) name: Option<String>,
    pub body: Body<'a>,
}

impl<'a> FunctionBuilder<'a> {
    pub fn new(params: &[ValType], results: &[ValType]) -> Self {
        Self {
            params: params.to_vec(),
            results: results.to_vec(),
            num_locals: 0,
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
    pub fn add_local(&mut self, ty: ValType) -> u32 {
        let index = self.params.len() as u32 + self.num_locals;
        self.num_locals += 1;
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

    pub fn local_get(&mut self, local_index: u32) -> &mut Self {
        self.body.add_instr(Operator::LocalGet { local_index });
        self
    }

    pub fn local_set(&mut self, local_index: u32) -> &mut Self {
        self.body.add_instr(Operator::LocalSet { local_index });
        self
    }

    pub fn i32_const(&mut self, value: i32) -> &mut Self {
        self.body.add_instr(Operator::I32Const { value });
        self
    }

    pub fn i64_const(&mut self, value: i64) -> &mut Self {
        self.body.add_instr(Operator::I64Const { value });
        self
    }

    pub fn f32_const(&mut self, value: f32) -> &mut Self {
        self.body.add_instr(Operator::F32Const {
            value: value.into(),
        });
        self
    }

    pub fn f64_const(&mut self, value: f64) -> &mut Self {
        self.body.add_instr(Operator::F64Const {
            value: value.into(),
        });
        self
    }

    // TODO: do some checking here?
    pub fn binop(&mut self, op: Operator<'a>) -> &mut Self {
        self.body.add_instr(op);
        self
    }

    // TODO: how to reason with BlockType, is there a better abstraction,
    // do they really matter?
    /// Inject a block statement. Indicates the start of a block
    pub fn block(&mut self, block_type: wasmparser::BlockType) -> &mut Self {
        self.body.add_instr(Operator::Block {
            blockty: block_type,
        });
        self
    }

    /// Inject a loop statement. Indicates the start of a loop
    // TODO: walrus does this with closures, do we want to swithc to that?
    // (same question for if_stmt)
    pub fn loop_stmt(&mut self, block_type: wasmparser::BlockType) -> &mut Self {
        self.body.add_instr(Operator::Loop {
            blockty: block_type,
        });
        self
    }

    pub fn if_stmt(&mut self, block_type: wasmparser::BlockType) -> &mut Self {
        self.body.add_instr(Operator::If {
            blockty: block_type,
        });
        self
    }

    pub fn else_stmt(&mut self) -> &mut Self {
        self.body.add_instr(Operator::Else);
        self
    }

    pub fn end(&mut self) -> &mut Self {
        self.body.add_instr(Operator::End);
        self
    }

    pub fn call(&mut self, function_index: u32) -> &mut Self {
        self.body.add_instr(Operator::Call { function_index });
        self
    }

    pub fn br(&mut self, relative_depth: u32) -> &mut Self {
        self.body.add_instr(Operator::Br { relative_depth });
        self
    }
}
