use crate::ir::function::FunctionModifier;
use crate::ir::id::{FunctionID, ImportsID, LocalID, TypeID};
use crate::ir::types::Body;
use crate::DataType;

/// Represents whether a function is a Local Function or an Imported Function. Also contains its type ID
#[derive(Debug, Clone)]
pub enum FuncKind<'a> {
    Local(LocalFunction<'a>),
    Import(ImportedFunction),
}

impl<'a> FuncKind<'a> {
    pub fn unwrap_local(&self) -> &LocalFunction<'a> {
        match &self {
            FuncKind::Local(l) => l,
            _ => panic!("Not a local function!"),
        }
    }

    pub fn get_type(&self) -> TypeID {
        match &self {
            FuncKind::Local(l) => l.ty_id,
            FuncKind::Import(i) => i.ty_id,
        }
    }
}

impl PartialEq for FuncKind<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FuncKind::Import(i1), FuncKind::Import(i2)) => i1.ty_id == i2.ty_id,
            (FuncKind::Local(l1), FuncKind::Local(l2)) => l1.ty_id == l2.ty_id,
            _ => false,
        }
    }
}

impl Eq for FuncKind<'_> {}

#[derive(Clone, Debug)]
pub struct LocalFunction<'a> {
    pub ty_id: TypeID,
    pub func_id: FunctionID,
    pub body: Body<'a>,
    pub args: Vec<LocalID>,
}

impl<'a> LocalFunction<'a> {
    pub fn new(
        type_id: TypeID,
        function_id: FunctionID,
        body: Body<'a>,
        args: Vec<LocalID>,
    ) -> Self {
        LocalFunction {
            ty_id: type_id,
            func_id: function_id,
            body,
            args,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImportedFunction {
    pub import_id: ImportsID,
    pub ty_id: TypeID,
}

impl ImportedFunction {
    pub fn new(id: ImportsID, type_id: TypeID) -> Self {
        ImportedFunction {
            import_id: id,
            ty_id: type_id,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function<'a> {
    pub(crate) kind: FuncKind<'a>,
    name: Option<String>,
}

impl<'a> Function<'a> {
    pub fn new(kind: FuncKind<'a>, name: Option<String>) -> Self {
        Function { kind, name }
    }

    pub fn get_type_id(&self) -> TypeID {
        match &self.kind {
            FuncKind::Local(l) => l.ty_id,
            FuncKind::Import(i) => i.ty_id,
        }
    }

    pub fn kind(&self) -> &FuncKind<'a> {
        &self.kind
    }

    pub fn unwrap_local(&'a self) -> &LocalFunction<'a> {
        match &self.kind {
            FuncKind::Local(l) => l,
            FuncKind::Import(_) => panic!("Imported Function!"),
        }
    }

    pub fn unwrap_local_mut(&mut self) -> &mut LocalFunction<'a> {
        match &mut self.kind {
            FuncKind::Local(ref mut l) => l,
            FuncKind::Import(_) => panic!("Imported Function!"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Functions<'a> {
    functions: Vec<Function<'a>>,
}

impl<'a> Functions<'a> {
    pub fn new(functions: Vec<Function<'a>>) -> Self {
        Functions { functions }
    }

    pub fn get_fn_by_id(&self, function_id: FunctionID) -> Option<Function> {
        if function_id < self.functions.len() as u32 {
            return Some(self.functions[function_id as usize].clone());
        }
        None
    }

    pub fn len(&self) -> usize {
        self.functions.len()
    }

    pub fn push(&mut self, func: Function<'a>) {
        self.functions.push(func);
    }

    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }

    pub fn get_type_id(&self, id: FunctionID) -> TypeID {
        self.functions[id as usize].get_type_id()
    }

    pub fn delete(&mut self, id: FunctionID) {
        if id < self.functions.len() as u32 {
            self.functions.remove(id as usize);
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Function<'a>> {
        self.functions.iter()
    }

    pub fn get(&self, function_id: FunctionID) -> &Function<'a> {
        &self.functions[function_id as usize]
    }

    pub fn get_mut(&mut self, function_id: FunctionID) -> &mut Function<'a> {
        &mut self.functions[function_id as usize]
    }

    pub fn get_kind(&self, function_id: FunctionID) -> &FuncKind<'a> {
        &self.functions[function_id as usize].kind
    }

    /// Get a function modifier from a function index
    pub fn get_fn_modifier<'b>(
        &'b mut self,
        func_id: FunctionID,
    ) -> Option<FunctionModifier<'b, 'a>> {
        // grab type and section and code section
        // let ty = self.functions.get(func_idx as usize)?;
        return match &mut self.functions.get_mut(func_id as usize)?.kind {
            FuncKind::Local(ref mut l) => Some(FunctionModifier::init(&mut l.body)),
            _ => None,
        };
        // None
    }

    pub fn unwrap_local(&'a mut self, function_id: FunctionID) -> &mut LocalFunction<'a> {
        match &mut self.functions[function_id as usize].kind {
            FuncKind::Local(ref mut l) => l,
            FuncKind::Import(_) => panic!("Imported Function!"),
        }
    }

    /// Get local Function ID by name
    pub fn get_local_fid_by_name(&self, name: &str) -> Option<FunctionID> {
        for (idx, func) in self.functions.iter().enumerate() {
            match &func.kind {
                FuncKind::Local(l) => match &l.body.name {
                    Some(n) => {
                        if n == name {
                            return Some(idx as FunctionID);
                        }
                    }
                    None => {}
                },
                _ => {}
            }
        }
        None
    }

    pub(crate) fn add_local(&'a mut self, func_idx: FunctionID, ty: DataType) -> LocalID {
        let func_body = &mut self.functions[func_idx as usize];
        match func_body.kind {
            FuncKind::Import(_) => panic!("Imported function"),
            FuncKind::Local(_) => {}
        }

        let func = &mut func_body.unwrap_local_mut();

        let num_params = func.args.len();

        let num_locals = func.body.num_locals;
        let index = num_params + num_locals;

        let len = func.body.locals.len();
        func.body.num_locals += 1;
        if len > 0 {
            let last = len - 1;
            if func.body.locals[last].1 == ty {
                func.body.locals[last].0 += 1;
            } else {
                func.body.locals.push((1, ty));
            }
        } else {
            // If no locals, just append
            func.body.locals.push((1, ty));
        }

        index as LocalID
    }

    pub fn set_local_fn_name(&mut self, func_idx: FunctionID, name: String) {
        match &mut self.functions[func_idx as usize].kind {
            FuncKind::Import(_) => panic!("is an imported function!"),
            FuncKind::Local(ref mut l) => l.body.name = Some(name),
        }
    }
}
