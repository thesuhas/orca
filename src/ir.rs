use wasmparser::{
    BinaryReaderError, Export, GlobalType, Import, MemoryType, Operator, Parser, Payload, RefType,
    SubType, TableType, ValType,
};

pub struct Global<'a> {
    pub ty: GlobalType,
    pub init_expr: Operator<'a>,
}

pub struct DataSegment<'a> {
    /// The kind of data segment.
    pub kind: DataSegmentKind<'a>,
    /// The data of the data segment.
    pub data: &'a [u8],
}

/// The kind of data segment.
#[derive(Debug, Clone)]
pub enum DataSegmentKind<'a> {
    /// The data segment is passive.
    Passive,
    /// The data segment is active.
    Active {
        /// The memory index for the data segment.
        memory_index: u32,
        /// The initialization operator for the data segment.
        offset_expr: Operator<'a>,
    },
}

pub enum ElementKind<'a> {
    Passive,
    Active {
        table_index: Option<u32>,
        offset_expr: Operator<'a>,
    },
    Declared,
}

pub enum ElementItems<'a> {
    Functions(Vec<u32>),
    ConstExprs {
        ty: RefType,
        exprs: Vec<Operator<'a>>,
    },
}

pub struct Body<'a> {
    /// Local variables of the function, given as tuples of (# of locals, type).
    /// Note that these do not include the function parameters which are given
    /// indices before the locals. So if a function has 2 parameters and a local
    /// defined here then local indices 0 and 1 will refer to the parameters and
    /// index 2 will refer to the local here.
    pub locals: Vec<(u32, ValType)>,
    pub instructions: Vec<Operator<'a>>,
}

pub struct Module<'a> {
    pub types: Vec<SubType>,
    pub imports: Vec<Import<'a>>,
    /// Mapping from function index to type index.
    pub functions: Vec<u32>,
    /// Each table has a type and optional initialization expression.
    pub tables: Vec<(TableType, Option<Operator<'a>>)>,
    pub memories: Vec<MemoryType>,
    pub globals: Vec<Global<'a>>,
    pub data: Vec<DataSegment<'a>>,
    pub data_count_section_exists: bool,
    pub exports: Vec<Export<'a>>,
    // Index of the start function.
    pub start: Option<u32>,
    pub elements: Vec<(ElementKind<'a>, ElementItems<'a>)>,
    pub code_sections: Vec<Body<'a>>,
    pub custom_sections: Vec<(&'a str, &'a [u8])>,
}

pub struct Component<'a> {
    /// Needs to contain:
    /// 1. Modules
    /// 2. Alias
    /// 3. Types
    /// 4. Import
    /// 5. Export
    /// TBD: Instances
}