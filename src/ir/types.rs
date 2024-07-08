use std::fmt;
use std::fmt::Formatter;
use wasmparser::{GlobalType, Operator, RefType, ValType};

#[derive(Debug, Clone)]
pub struct Global<'a> {
    pub ty: GlobalType,
    // TODO: We might want to build our own representation of econstant expression
    // seee https://docs.rs/walrus/latest/src/walrus/const_expr.rs.html#13-22
    pub init_expr: wasmparser::ConstExpr<'a>,
}

#[derive(Debug, Clone)]
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
        offset_expr: wasmparser::ConstExpr<'a>,
    },
}

#[derive(Debug, Clone)]
pub enum ElementKind<'a> {
    Passive,
    Active {
        table_index: Option<u32>,
        offset_expr: wasmparser::ConstExpr<'a>,
    },
    Declared,
}

#[derive(Debug, Clone)]
pub enum ElementItems<'a> {
    Functions(Vec<u32>),
    ConstExprs {
        ty: RefType,
        exprs: Vec<wasmparser::ConstExpr<'a>>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InstrumentType {
    InstrumentBefore,
    InstrumentAfter,
    InstrumentAlternate,
    NotInstrumented,
}

impl fmt::Display for InstrumentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            InstrumentType::InstrumentBefore => write!(f, "Instrument Before"),
            InstrumentType::InstrumentAfter => write!(f, "Instrument After"),
            InstrumentType::InstrumentAlternate => write!(f, "Instrument Alternate"),
            InstrumentType::NotInstrumented => write!(f, "Not Instrumented"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Body<'a> {
    /// Local variables of the function, given as tuples of (# of locals, type).
    /// Note that these do not include the function parameters which are given
    /// indices before the locals. So if a function has 2 parameters and a local
    /// defined here then local indices 0 and 1 will refer to the parameters and
    /// index 2 will refer to the local here.
    pub locals: Vec<(u32, ValType)>,
    pub instructions: Vec<(Operator<'a>, InstrumentType)>,
}