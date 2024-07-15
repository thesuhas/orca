//! Intermediate Representation of a fucntion

use crate::ir::module::Module;
use crate::ir::types::Body;
use wasmparser::ValType;


// TODO: probably need better reasoning with lifetime here
struct FunctionBuilder <'a> {
    pub(crate) ty: u32, // type index
    pub(crate) entry: Option<u32>, // offset of 
    pub(crate) name: Option<String>,
    pub(crate) body: Body <'a>,
}

impl <'a> FunctionBuilder <'a> {
    pub fn new(module: &mut Module, params: &[ValType], results: &[ValType]) -> Self {
        let ty = module.add_type(params, results);
        Self {
            ty,
            entry: None,
            name: None,
            body: Body::new(),
        }
    }
}


